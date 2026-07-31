use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    estructuras::{
        config::{Config, RunnerOption},
        desktop,
        game::GameConfig,
        games::Games,
    },
    libs::{
        cli, dialogos,
        icon::IconProcesor,
        importer::{export_game, import_game},
        installer::Runnerinstaller,
        utils::notify,
        vars::Variables,
    },
};
use iced::{
    Color, Element, Length, Theme, theme,
    widget::{
        button, checkbox, column, grid, image, pick_list, row, rule, scrollable, space, text,
        text_input,
    },
    window::{self},
};
use iced_aw::context_menu;
mod estructuras;
mod libs;
#[derive(Debug, Clone)]
pub enum Message {
    Warning(String),
    Save,
    Play,
    Delete,
    Shortcut,
    RecreateShortcuts,
    CreateShortcut,
    ImportGame,
    ImportRunner,
    ExportGame,
    Select(String, String),
    SelectAction(String, &'static str),
    ChangeView(i32, &'static str),
    InputEdit(&'static str, String),
    PickFile(&'static str),
    SelectRunner(String),
    PickCheckVar(String, bool),
    PickCheckOption(String, String, bool),
    PickInput(String, String, String),
    PickFolder(String, String),
}

pub fn game_card<'a>(id: &'a str, nombre: &'a str, portada: &'a str) -> Element<'a, Message> {
    let imagen = context_menu::ContextMenu::new(
        button(image(portada).width(200).height(200))
            .on_press(Message::Select(id.to_string(), nombre.to_string())),
        || {
            column![
                button("▶ Jugar ").on_press(Message::SelectAction(id.to_string(), "play")),
                button("✎ Editar").on_press(Message::SelectAction(id.to_string(), "edit")),
                button("✖ Eliminar").on_press(Message::SelectAction(id.to_string(), "delete")),
                button("★ Crear atajo").on_press(Message::SelectAction(id.to_string(), "desk")),
                button("🗁 Abrir carpeta")
                    .on_press(Message::SelectAction(id.to_string(), "openfolder")),
                button("⚙ Exportar config")
                    .on_press(Message::SelectAction(id.to_string(), "export")),
            ]
            .into()
        },
    );
    column![
        imagen,
        button(text(format!("▶ {}", nombre)))
            .on_press(Message::SelectAction(id.to_string(), "play"))
            .width(220) // button("▶ Jugar")
                        //     .on_press(Message::SelectPlay(id.to_string()))
                        //     .width(200)
    ]
    .into()
}

fn archivos_en_carpeta(path: &String) -> anyhow::Result<Vec<String>> {
    let mut archivos = Vec::new();
    let paths = fs::read_dir(path)?;
    for entry in paths {
        let entry = entry?;
        if let Some(nombre) = entry.path().file_name() {
            archivos.push(nombre.to_string_lossy().into_owned());
        }
    }
    Ok(archivos)
}

#[derive(Default)]
struct EditView {
    name: String,
    cmd: String,
    path: String,
    icon: String,
    args: String,
    vars: String,
    runners: Vec<String>,
    runner: String,
    mode: String,
}

impl EditView {
    pub fn new(options: &Config) -> Self {
        let runners = options.runners.keys().cloned().collect();
        Self {
            runners: runners,
            ..Default::default()
        }
    }
}

fn option_to_widget<'a>(
    name: &'a String,
    option: &'a RunnerOption,
    modo: &'a str,
) -> Element<'a, Message> {
    let mut r = row![];
    if modo != "var" {
        r =
            r.push(checkbox(option.enable).on_toggle(move |val| {
                Message::PickCheckOption(name.clone(), modo.to_string(), val)
            }));
    }
    r = r.push(text(name.as_str()));
    if option.mode == "input" {
        r = r.push(
            text_input(&name, &option.input)
                .on_input(|val| Message::PickInput(name.clone(), val, modo.to_string())),
        );
    }
    if option.mode == "file_names" {
        let mut files = Vec::new();
        let vars = Variables::default().apply(&option.cmd);
        if let Ok(f) = archivos_en_carpeta(&vars) {
            files = f;
        };
        r = r.push(pick_list(files, Some(option.input.clone()), |val| {
            Message::PickInput(name.clone(), val, "var".to_string())
        }));
    }
    if option.mode == "folder" {
        r = r.push(
            text_input("carpeta", &option.input)
                .on_input(|val| Message::PickInput(name.clone(), val, "var".to_string())),
        );
        r = r.push(
            button("🗁 Selecionar carpeta")
                .on_press(Message::PickFolder(name.clone(), "var".to_string())),
        );
    }
    r.spacing(10).into()
}

fn set_option(options: &mut HashMap<String, RunnerOption>, id: &str, value: bool) {
    if let Some(option) = options.get_mut(id) {
        option.enable = value;
    }
}

fn set_pickinput(options: &mut HashMap<String, RunnerOption>, id: &str, value: String) {
    if let Some(option) = options.get_mut(id) {
        option.input = value;
    }
}

struct App {
    gamesdata: Games,
    optionsdata: Config,
    logopath: String,
    nombre: String,
    view: i32,
    warning: String,
    editview: EditView,
    vars: Variables,
    iconfinder: IconProcesor,
}

impl Default for App {
    fn default() -> Self {
        let var = Variables::default();
        let gamesdata2 = Games::default();
        let optionsdata2 = fs::read_to_string(format!("{}/options.json", var.CONFIG)).unwrap();
        let options = serde_json::from_str(&optionsdata2).unwrap();
        Self {
            gamesdata: gamesdata2,
            editview: EditView::new(&options),
            optionsdata: options,
            logopath: format!("{}/cat_games_launcher.png", var.ICONS),
            nombre: String::new(),
            view: 0,
            warning: String::new(),
            vars: var,
            iconfinder: IconProcesor::default(),
        }
    }
}

impl App {
    pub fn formatedit(&mut self) {
        for (_, g) in self.optionsdata.global.iter_mut() {
            g.enable = false;
            g.input = String::new();
        }
        for (_, runner) in self.optionsdata.runners.iter_mut() {
            for (_, val) in runner.options.iter_mut() {
                val.enable = false;
                val.input = String::new();
            }
        }
        self.editview = EditView {
            runners: self.editview.runners.clone(),
            ..Default::default()
        };
    }
    pub fn enteredit(&mut self, num: i32, dat: String) {
        self.formatedit();
        self.view = num;
        self.editview.mode = dat;
        if let Some(game) = self.gamesdata.configs.get(&self.gamesdata.play) {
            self.editview.name = game.name.clone();
            self.editview.icon = game.icon.clone();
            self.editview.cmd = game.bin.clone();
            self.editview.path = game.cwd.clone();
            self.editview.vars = game.prefix.clone();
            self.editview.args = game.args.clone();
            self.editview.runner = game.runner_name.clone();
            for (i, v) in &game.global {
                if let Some(g) = self.optionsdata.global.get_mut(i) {
                    g.enable = v.enable.clone();
                    g.input = v.input.clone();
                }
            }
            if let Some(runner) = self.optionsdata.runners.get_mut(&game.runner_name) {
                for (i, v) in &game.options {
                    if let Some(val) = runner.options.get_mut(i) {
                        val.enable = v.enable;
                        val.input = v.input.clone();
                    };
                }
            }
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        if self.view == 1 {
            let mut globales = column![];
            for (i, v) in &self.optionsdata.global {
                globales = globales.push(option_to_widget(i, v, "global"));
            }
            let mut vars = column![];
            let mut options = column![];
            if let Some(i) = self.optionsdata.runners.get(&self.editview.runner) {
                for (i, opt) in &i.options {
                    if opt.kind == "var" {
                        vars = vars.push(option_to_widget(i, opt, "var"));
                    } else {
                        options = options.push(option_to_widget(i, opt, "runner"));
                    }
                }
            }

            column![
                button(" ↩ Atras ").on_press(Message::ChangeView(0, "")),
                button("🖫 Guardar")
                    .on_press(Message::Save)
                    .width(Length::Fill),
                pick_list(
                    self.editview.runners.clone(),
                    Some(self.editview.runner.clone()),
                    Message::SelectRunner
                )
                .width(Length::Fill),
                text_input("Nombre", &self.editview.name)
                    .on_input(|texto| Message::InputEdit("name", texto)),
                scrollable(
                    column![
                        row![
                            image(&self.editview.icon).width(100).height(100),
                            column![
                                text_input("Icono", &self.editview.icon)
                                    .on_input(|texto| Message::InputEdit("icon", texto)),
                                button("🗋 Seleciona icono").on_press(Message::PickFile("icon")),
                                button("⌕ Buscar icono").on_press(Message::PickFile("detecticon"))
                            ]
                        ],
                        row![
                            text_input("Ejecutable", &self.editview.cmd)
                                .on_input(|texto| Message::InputEdit("bin", texto)),
                            button("🗋 Selecionar archivo").on_press(Message::PickFile("bin"))
                        ],
                        row![
                            text_input("carpeta", &self.editview.path)
                                .on_input(|texto| Message::InputEdit("path", texto)),
                            button("🗁 Selecionar carpeta").on_press(Message::PickFile("path"))
                        ],
                        text_input("Argumentos", &self.editview.args)
                            .on_input(|texto| Message::InputEdit("args", texto)),
                        text_input("Variables", &self.editview.vars)
                            .on_input(|texto| Message::InputEdit("vars", texto)),
                        vars.spacing(10),
                        rule::horizontal(1),
                        globales.spacing(10),
                        text!("Opciones: "),
                        options.spacing(10),
                    ]
                    .spacing(10)
                ),
            ]
            .spacing(10)
            .padding(10)
            .into()
        } else if self.view == 2 {
            return column![
                button(" ↩ Atras ").on_press(Message::ChangeView(0, "")),
                rule::horizontal(1),
                text("Atajos"),
                button("★ Crear atajo del launcher").on_press(Message::CreateShortcut),
                button("★ Actualizar todos los launchers").on_press(Message::RecreateShortcuts),
                rule::horizontal(1),
                text("Importacion"),
                button("⎙ Importar Juego").on_press(Message::ImportGame),
                button("⎙ Importar Runner").on_press(Message::ImportRunner)
            ]
            .spacing(10)
            .padding(10)
            .into();
        } else {
            let mut juegos = grid![].fluid(270);
            for (i, v) in &self.gamesdata.configs {
                if Path::new(&v.icon).exists() {
                    juegos = juegos.push(game_card(&i.as_str(), &v.name.as_str(), &v.icon));
                } else {
                    juegos = juegos.push(game_card(&i.as_str(), &v.name.as_str(), &self.logopath));
                }
            }
            column![
                text!("Cat games Launcher").size(25),
                space().height(10),
                row![
                    text!("{}", self.nombre).width(Length::Fill),
                    text!("{}", self.gamesdata.play).width(Length::Fill),
                    text!("{}", self.warning).width(Length::Fill),
                ],
                rule::horizontal(1),
                row![
                    button("▶ Jugar").on_press(Message::Play),
                    button("✚ Añadir").on_press(Message::ChangeView(1, "add")),
                    button("✖ Eliminar").on_press(Message::Delete),
                    button("✎ Editar").on_press(Message::ChangeView(1, "edit")),
                    button("★ Crear atajo").on_press(Message::Shortcut),
                    button("⚙ Exportar config").on_press(Message::ExportGame),
                    button("⚒︎ Extras").on_press(Message::ChangeView(2, "Extras")),
                ]
                .spacing(10)
                .padding(10),
                scrollable(juegos.spacing(10)),
            ]
            .padding(20)
            .into()
        }
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Save => {
                let path = format!("{}/games.json", Variables::CONFIG());
                let juego = GameConfig::new(
                    &self.optionsdata,
                    &self.editview.runner,
                    &self.editview.name,
                    &self.editview.cmd,
                    &self.editview.path,
                    &self.editview.args,
                    &self.editview.vars,
                    &self.editview.icon,
                );
                let check;
                if self.editview.mode == "edit" {
                    check =
                        self.gamesdata
                            .add_game(&path, &juego, Some(self.gamesdata.play.clone()));
                } else {
                    check = self.gamesdata.add_game(&path, &juego, None);
                }
                if check.is_ok() {
                    println!("Proceso completado...");
                    self.view = 0;
                    self.warning = "🖫 Guardado!".to_string();
                }
            }
            Message::Select(id, nombre) => {
                self.gamesdata.play = id;
                self.nombre = nombre;
            }
            Message::Play => {
                self.gamesdata.play();
            }
            Message::Shortcut => {
                if self.gamesdata.shortcut().is_ok() {
                    notify("⎙ Atajo creado correctamente.");
                };
            }
            Message::ChangeView(num, dat) => {
                if num == 1 {
                    if dat == "edit" {
                        if !self.gamesdata.play.is_empty() {
                            self.enteredit(num, dat.to_string());
                        } else {
                            self.warning = "Error: Seleciona un juego antes.".to_string();
                        }
                    } else {
                        self.view = num;
                        self.editview.mode = dat.to_string();
                        self.formatedit();
                    }
                } else {
                    self.view = num;
                }
            }
            Message::Warning(texto) => {
                self.warning = texto;
            }
            Message::InputEdit(id, input) => {
                if id == "name".to_string() {
                    self.editview.name = input;
                } else if id == "bin" {
                    self.editview.cmd = input;
                } else if id == "path" {
                    self.editview.path = input;
                } else if id == "icon" {
                    self.editview.icon = input;
                } else if id == "args" {
                    self.editview.args = input;
                } else if id == "vars" {
                    self.editview.vars = input;
                }
            }
            Message::PickFile(id) => {
                if id == "icon" {
                    let file = dialogos::iconpicker();
                    if !file.is_empty() {
                        self.editview.icon = file.clone();
                    }
                } else if id == "bin" {
                    let file = dialogos::filepicker("ejecutable", &["*"]);
                    if !file.is_empty() {
                        self.editview.cmd = file.clone();
                        let filepath = PathBuf::from(&file);
                        if self.editview.path.is_empty() {
                            self.editview.path =
                                filepath.parent().unwrap().to_string_lossy().to_string();
                        };
                        if self.editview.name.is_empty() {
                            self.editview.name =
                                filepath.file_stem().unwrap().to_string_lossy().to_string();
                        }
                        if self.editview.icon.is_empty() {
                            if file.ends_with(".exe") {
                                self.editview.icon = self.iconfinder.icon_exe(&file);
                            } else {
                                self.editview.icon =
                                    self.iconfinder.locateicon(&file, &self.editview.name)
                            }
                        }
                    }
                } else if id == "path" {
                    let path = dialogos::folderpicker();
                    if !path.is_empty() {
                        self.editview.path = path;
                    }
                } else if id == "detecticon" {
                    let icon = self
                        .iconfinder
                        .locateicon(&self.editview.cmd, &self.editview.name);
                    if !icon.is_empty() {
                        self.editview.icon = icon;
                    }
                }
            }
            Message::SelectRunner(runner) => {
                self.editview.runner = runner;
            }
            Message::PickCheckOption(id, modo, value) => {
                if modo == "global" {
                    set_option(&mut self.optionsdata.global, &id, value);
                } else if let Some(runner) = self.optionsdata.runners.get_mut(&self.editview.runner)
                {
                    set_option(&mut runner.options, &id, value);
                }
            }
            Message::PickCheckVar(id, value) => {
                if let Some(opt) = self.optionsdata.global.get_mut(&id) {
                    opt.enable = value;
                };
            }
            Message::Delete => {
                if self.gamesdata.play.is_empty() {
                    self.warning = "Seleciona un juego.".to_string()
                } else {
                    self.gamesdata.delete_game(&self.gamesdata.play.clone());
                }
            }
            Message::PickInput(id, value, mode) => {
                if mode == "global" {
                    set_pickinput(&mut self.optionsdata.global, &id, value);
                } else if let Some(runner) = self.optionsdata.runners.get_mut(&self.editview.runner)
                {
                    set_pickinput(&mut runner.options, &id, value);
                }
            }
            Message::PickFolder(id, mode) => {
                let value = dialogos::folderpicker();
                if mode == "global" {
                    set_pickinput(&mut self.optionsdata.global, &id, value);
                } else if let Some(runner) = self.optionsdata.runners.get_mut(&self.editview.runner)
                {
                    set_pickinput(&mut runner.options, &id, value);
                }
            }
            Message::CreateShortcut => {
                let mut atajo = desktop::Desktop::default();
                atajo.name = "Cat Games Launcher".to_string();
                atajo.exec = env::current_exe().unwrap().to_string_lossy().to_string();
                atajo.comment = "Lanzador de juegos hecho en rust.".to_string();
                atajo.icon = "cat_games_launcher".to_string();
                if atajo
                    .save(
                        format!(
                            "{}/.local/share/applications/cat_launcher.desktop",
                            &self.vars.HOME
                        )
                        .as_str(),
                    )
                    .is_ok()
                {
                    notify("Atajo creado.");
                }
                {};
            }
            Message::ExportGame => {
                if !&self.gamesdata.play.is_empty() {
                    if let Some(game) = self.gamesdata.configs.get(&self.gamesdata.play) {
                        if export_game(game, &game.cwd).is_ok() {
                            notify("Juego exportado.");
                        };
                    };
                } else {
                    self.warning = "ⓘ Debes selecionar un juego.".to_string()
                }
            }
            Message::ImportGame => {
                let f = dialogos::filepicker("juego (.cat_game)", &[&"cat_game", "json"]);
                if !f.is_empty() {
                    if import_game(&f, &mut self.gamesdata).is_ok() {
                        notify("Juego importado correctamente.");
                        self.view = 0;
                        self.warning = "Juego importado correctamente.".to_string()
                    };
                }
            }
            Message::ImportRunner => {
                let f = dialogos::filepicker("runner (.cat_runner)", &["cat_runner", "json"]);
                if !f.is_empty() {
                    if let Ok(runner_installer) = Runnerinstaller::new(&f) {
                        if runner_installer.install().is_ok() {
                            if runner_installer.import(&mut self.optionsdata).is_ok() {
                                self.view = 0;
                                notify("Runner importado correctamente.");
                                self.editview.runners =
                                    self.optionsdata.runners.keys().cloned().collect();
                                self.warning = "Runner importado.".to_string();
                            };
                        };
                    };
                };
            }
            Message::RecreateShortcuts => {
                if let Ok(desks) = fs::read_dir(&self.vars.DESKTOP) {
                    for desk in desks {
                        if let Ok(desk) = desk {
                            let name = desk
                                .path()
                                .file_stem()
                                .unwrap()
                                .to_string_lossy()
                                .to_string();
                            if let Some(game) = self.gamesdata.configs.get(&name) {
                                game.shortcut(&name);
                                println!("ⓘ {}: {} Atajo creado.", game.name, &name);
                            }
                        }
                    }
                }
            }
            Message::SelectAction(id, mode) => {
                self.gamesdata.play = id.clone();
                if mode == "play" {
                    self.gamesdata.play();
                } else if mode == "edit" {
                    self.enteredit(1, "edit".to_string());
                } else if mode == "delete" {
                    self.gamesdata.delete_game(&id);
                } else if mode == "export" {
                    if let Some(game) = self.gamesdata.configs.get(&self.gamesdata.play) {
                        if export_game(game, &game.cwd).is_ok() {
                            notify("⎙ Juego exportado.");
                        };
                    };
                } else if mode == "desk" {
                    if self.gamesdata.shortcut().is_ok() {
                        notify("⎙ Atajo creado correctamente.");
                    };
                } else if mode == "openfolder" {
                    if let Some(game) = self.gamesdata.configs.get(&self.gamesdata.play) {
                        Command::new("xdg-open").arg(&game.cwd).spawn().ok();
                    };
                }
            }
        }
    }
}

fn main() -> iced::Result {
    let vars = Variables::default();
    Variables::gen_path().ok();
    let argumentos: Vec<String> = env::args().skip(1).collect();
    if argumentos.len() > 0 {
        cli::args_parse(argumentos).unwrap();
        return iced::Result::Ok(());
    }
    let logopng = PathBuf::from(format!("{}/cat_games_launcher.png", vars.ICONS));
    if !logopng.exists() {
        println!("Copiando logo.png....");
        fs::write(logopng, include_bytes!("assets/cat_games_launcher.png")).ok();
    };
    let configdefault = PathBuf::from(format!("{}/options.json", vars.CONFIG));
    if !configdefault.exists() {
        println!("Copiando runners por defecto....");
        fs::write(configdefault, include_bytes!("assets/options.json")).ok();
    };
    let icon =
        window::icon::from_file_data(include_bytes!("assets/cat_games_launcher.png"), None).ok();

    let tema = Theme::custom(
        "Mi Tema",
        theme::Palette {
            background: Color::from_rgb8(25, 25, 25),
            text: Color::WHITE,
            primary: Color::from_rgb8(53, 53, 53),
            success: Color::from_rgb8(50, 200, 100),
            danger: Color::from_rgb8(220, 80, 80),
            warning: Color::from_rgb8(220, 80, 80),
        },
    );
    return iced::application(App::default, App::update, App::view)
        .title("Cat Games Launcher")
        .theme(tema)
        .window(window::Settings {
            icon: icon,
            ..Default::default()
        })
        .run();
    // iced::run(App::update, App::view)
}
