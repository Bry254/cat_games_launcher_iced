use std::{collections::HashMap, fs};

use anyhow::Ok;

use crate::{
    GameConfig,
    libs::{utils::notify, vars::Variables},
};

pub fn args_parse(args: Vec<String>) -> anyhow::Result<()> {
    let variables = Variables::default();
    let data = fs::read_to_string(format!("{}/games.json", variables.CONFIG))?;
    let games: HashMap<String, GameConfig> = serde_json::from_str(&data.as_str())?;
    if args[0] == "run" {
        let mut id = args[1].clone();
        if !id.contains("cat_game") {
            id = format!("cat_game{id}");
        }
        println!("Iniciando Juego: {}", args[1]);
        if let Some(juego) = games.get(&id) {
            notify("▶ Juego iniciado.");
            GameConfig::execute(GameConfig::gen_cmd(juego), juego.cwd.clone()).ok();
        }
    }
    if args[0] == "ls" {
        for (i, v) in games.iter() {
            println!("{} : {}", i, v.name);
        }
    }
    return Ok(());
}
