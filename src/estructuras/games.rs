use anyhow::Ok;

use crate::libs::{
    dialogos,
    vars::{self, Variables},
};

use super::game::GameConfig;
use std::{collections::HashMap, fs, path::Path, process::Command};

pub struct Games {
    pub configs: HashMap<String, GameConfig>,
    pub play: String,
    // pub child: Option<std::process::Child>,
}

impl Default for Games {
    fn default() -> Self {
        let var = Variables::default();
        let filepath = format!("{}games.json", var.CONFIG);
        let mut config = HashMap::default();
        if Path::new(&filepath).exists() {
            let data = fs::read_to_string(filepath).unwrap();
            config = serde_json::from_str(&data).unwrap();
        } else {
            println!("Archivo de juegos no existe creando uno...");
            fs::write(filepath, "{}").unwrap();
        }
        Self {
            configs: config,
            play: String::new(),
            // child: None,
        }
    }
}

impl Games {
    pub fn play(&mut self) {
        if let Some(game) = self.configs.get(&self.play) {
            game.clone().play().ok();
            println!("Iniciado");
        }
    }
    pub fn shortcut(&mut self) -> anyhow::Result<()> {
        if let Some(game) = self.configs.get(&self.play) {
            return game.clone().create_shortcut(&self.play);
        }
        Ok(())
    }
    pub fn add_game(
        &mut self,
        path: &String,
        juego: &GameConfig,
        game_id: Option<String>,
    ) -> anyhow::Result<()> {
        let mut id = String::from("cat_game1");
        let mut counter = 1;
        while self.configs.contains_key(&id) {
            id = format!("cat_game{}", counter);
            counter += 1;
        }
        if let Some(gid) = game_id {
            id = gid.clone();
            println!("[Modo edición]");
        }
        println!("Id de juego: {id}");
        self.configs.insert(id.clone(), juego.clone());
        let data = serde_json::to_string_pretty(&self.configs)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn delete_game(&mut self, game_id: &String) {
        if dialogos::confirm(
            "Confirmar",
            &format!(
                "¿Eliminar el juego \"{}\"?",
                self.configs.get(game_id).unwrap().name
            ),
        ) {
            let vars = vars::Variables::default();
            self.configs.remove(game_id);
            let data = serde_json::to_string_pretty(&self.configs).unwrap();
            let path = format!("{}/games.json", vars.CONFIG);
            fs::remove_file(format!("{}/{}.desktop", vars.DESKTOP, game_id)).ok();
            Command::new("update-desktop-database")
                .arg(format!("{}/.local/share/applications", vars.HOME))
                .spawn()
                .ok();
            fs::write(path, data).unwrap();
        }
    }
}
