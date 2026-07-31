use std::{env, fs, path::Path, process::Command};

use crate::{
    estructuras::{game::GameConfig, games::Games},
    libs::dialogos,
};

pub fn read_file(path: &str) -> anyhow::Result<String> {
    let data = fs::read_to_string(path)?;
    return Ok(data.to_string());
}

pub fn import_game(file: &String, games: &mut Games) -> anyhow::Result<()> {
    let path = Path::new(file)
        .parent()
        .expect("")
        .to_string_lossy()
        .to_string();
    let data = read_file(file)?;
    let mut game: GameConfig = serde_json::from_str(&data)?;
    game.cwd = path.clone();
    game.bin = game.bin.replace("$cwd", path.as_str());

    game.icon = game.icon.replace("$cwd", path.as_str());
    let mut id = None;
    for (i, g) in games.configs.iter() {
        if g.name == game.name {
            if dialogos::confirm("El juego ya existe", "Reemplazar?") {
                id = Some(i.clone());
                break;
            }
        }
    }
    Command::new("chmod").args(["+x", &game.bin]).spawn().ok();
    games.add_game(
        &format!(
            "{}/.config/cat_games_launcher/games.json",
            env::home_dir().unwrap().to_string_lossy().to_string()
        ),
        &game,
        id,
    )?;
    Ok(())
}

pub fn export_game(game: &GameConfig, path: &String) -> anyhow::Result<()> {
    let mut game = game.clone();
    let cwd = game.cwd.clone();
    if !game.icon.is_empty() {
        if let Some(name) = Path::new(&game.icon).file_name() {
            println!("Copiando icono del juego...");
            let target = format!("{}/{}", game.cwd, name.to_string_lossy());
            std::fs::copy(&game.icon, target)?;
            game.icon = format!("$cwd/{}", name.to_string_lossy());
        }
    }
    game.bin = game.bin.replace(cwd.as_str(), "$cwd");
    game.cwd = String::from("$cwd");
    let data = serde_json::to_string_pretty(&game)?;
    fs::write(format!("{path}/config.cat_game"), data)?;
    Ok(())
}
