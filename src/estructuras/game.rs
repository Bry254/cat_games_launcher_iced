use super::desktop::Desktop;
use crate::estructuras::config::{Config, RunnerOption};
use crate::libs::utils::{notify, process_options};
use crate::libs::vars::Variables;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{collections::HashMap, env};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameConfig {
    pub name: String,
    #[serde(default)]
    pub icon: String,
    pub bin: String,
    pub cwd: String,
    pub args: String,
    pub prefix: String,
    pub global: HashMap<String, RunnerOption>,
    pub options: HashMap<String, RunnerOption>,
    pub runner_name: String,
    pub command_base: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installer: Option<String>,
}

impl GameConfig {
    pub fn shortcut(&self, id: &String) -> Desktop {
        let mut icon = "cat_games_launcher".to_string();
        if !self.icon.is_empty() {
            icon = self.icon.clone();
        }
        let mut short = Desktop::default();
        short.name = self.name.clone();
        short.comment = format!("Juego {} cat games launcher.", self.name);
        short.exec = format!(
            "{} run {}",
            env::current_exe().unwrap().to_string_lossy().to_string(),
            id
        );
        short.icon = icon;
        short.path = self.cwd.clone();
        println!("{}", &short.clone().to_string());
        return short;
    }

    pub fn new(
        config: &Config,
        runner_name: &String,
        name: &String,
        add_bin: &String,
        cwd: &String,
        args: &String,
        prefix: &String,
        icon: &String,
    ) -> GameConfig {
        let runner = config.runners.get(runner_name).unwrap();
        let mut local_runner_options = HashMap::new();
        for (i, v) in &runner.options {
            if v.enable || v.kind == "var" {
                local_runner_options.insert(i.clone(), v.clone());
            }
        }
        let mut local_global_options = HashMap::new();
        for (i, v) in &config.global {
            if v.enable {
                local_global_options.insert(i.clone(), v.clone());
            }
        }
        let data = GameConfig {
            name: name.clone(),
            bin: add_bin.clone(),
            cwd: cwd.clone(),
            args: args.clone(),
            prefix: prefix.clone(),
            runner_name: runner_name.clone(),
            global: local_global_options,
            options: local_runner_options,
            command_base: runner.command.clone(),
            icon: icon.clone(),
            installer: Some(String::new()),
        };
        println!("{:#?}", data);
        return data;
    }

    pub fn gen_cmd(game: &GameConfig) -> String {
        // println!("{:?}", game);
        let mut cmd = game.command_base.clone();
        let mut options = vec![];
        let mut variables = vec![];
        let globales = game.global.values().cloned().collect();
        for (_, v) in &game.options {
            if v.kind == "var" {
                variables.push(v.clone());
            } else {
                options.push(v.clone());
            }
        }
        let (global_prefix, global_commmand, global_args) = process_options(&globales);
        let (option_prefix, option_commmand, option_args) = process_options(&options);
        let command = global_commmand + &option_commmand;
        let prefix = global_prefix + &option_prefix;
        let args = global_args + &option_args;

        for i in variables {
            // println!("{} -> {}", i.var, i.input);
            cmd = cmd.replace(&i.var, &i.input);
        }
        cmd = cmd.replace("$add_bin", &format!("\"{}\"", game.bin).to_string());
        let vars = Variables::default();
        return vars.apply(
            &format!(
                "{command} env {prefix} {} {cmd} {} {args}",
                game.prefix, game.args
            )
            .to_string(),
        );
    }

    pub fn execute(cmd: String, cwd: String) -> anyhow::Result<std::process::Child> {
        #[cfg(target_os = "linux")]
        {
            println!("Ejecutando comando: {}", cmd);
            use std::process::Command;
            let pid = Command::new("setsid")
                .arg("bash")
                .current_dir(cwd)
                .arg("-c")
                .arg(cmd)
                .spawn()?;
            println!("Proceso iniciado con PID: {}", pid.id());
            return Ok(pid);
        }
    }

    pub fn play(self) -> anyhow::Result<()> {
        notify(format!("{} iniciado", self.name).as_str());
        GameConfig::execute(GameConfig::gen_cmd(&self), self.cwd.clone())?;
        Ok(())
    }

    pub fn create_shortcut(self, id: &String) -> anyhow::Result<()> {
        let home = env::home_dir().unwrap().to_string_lossy().to_string();
        let path = format!("{home}/.local/share/applications/cat_games");
        let file = format!("{path}/{id}.desktop");
        println!("Archivo {file} creado.");
        fs::create_dir_all(&path)?;
        self.shortcut(id).save(&file)?;
        Ok(())
    }
}
