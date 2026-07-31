use std::{
    env::{self, home_dir},
    fs,
};
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct Variables {
    pub HOME: String,
    pub CONFIG: String,
    pub DESKTOP: String,
    pub ICONS: String,
    pub lOCALDATA: String,
    pub TMP: String,
}
impl Default for Variables {
    fn default() -> Self {
        let home = env::home_dir().unwrap().to_string_lossy().to_string();
        let config = format!("{home}/.config/cat_games_launcher/");
        let desktop = format!("{home}/.local/share/applications/cat_games/");
        let icons = format!("{home}/.local/share/icons/hicolor/128x128/apps/");
        let localdata = format!("{home}/.local/share/cat_games_launcher/");
        fs::create_dir_all(&config).ok();
        fs::create_dir_all(&desktop).ok();
        fs::create_dir_all(&icons).ok();
        fs::create_dir_all(&localdata).ok();
        return Self {
            HOME: home,
            CONFIG: config,
            DESKTOP: desktop,
            ICONS: icons,
            lOCALDATA: localdata,
            TMP: env::temp_dir().to_string_lossy().to_string(),
        };
    }
}
impl Variables {
    pub fn gen_path() -> anyhow::Result<()> {
        let home = env::home_dir().unwrap().to_string_lossy().to_string();
        let config = format!("{home}/.config/cat_games_launcher/");
        let desktop = format!("{home}/.local/share/applications/cat_games/");
        let icons = format!("{home}/.local/share/icons/hicolor/128x128/apps/");
        let localdata = format!("{home}/.local/share/cat_games_launcher/");
        fs::create_dir_all(&config)?;
        fs::create_dir_all(&desktop)?;
        fs::create_dir_all(&icons)?;
        fs::create_dir_all(&localdata)?;
        Ok(())
    }
    pub fn apply(&self, data: &String) -> String {
        return data
            .clone()
            .replace("$HOME", &self.HOME)
            .replace("$localdata", &self.lOCALDATA)
            .replace("$tmp", &self.TMP);
    }
    #[allow(non_snake_case)]
    pub fn HOME() -> String {
        return home_dir().unwrap().to_string_lossy().to_string();
    }
    #[allow(non_snake_case)]
    pub fn CONFIG() -> String {
        let home = home_dir().unwrap().to_string_lossy().to_string();
        return format!("{home}/.config/cat_games_launcher/");
    }
    #[allow(non_snake_case)]
    pub fn DESKTOP() -> String {
        let home = home_dir().unwrap().to_string_lossy().to_string();
        return format!("{home}/.local/share/applications/cat_games/");
    }
    #[allow(non_snake_case)]
    pub fn ICONS() -> String {
        let home = home_dir().unwrap().to_string_lossy().to_string();
        return format!("{home}/.local/share/icons/hicolor/128x128/apps/");
    }
    #[allow(non_snake_case)]
    pub fn lOCAL() -> String {
        let home = home_dir().unwrap().to_string_lossy().to_string();
        return format!("{home}/.local/share/cat_games_launcher/");
    }
}
