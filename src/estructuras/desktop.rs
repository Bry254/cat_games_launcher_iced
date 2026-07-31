use std::{env, process::Command};

#[derive(Debug, Clone)]
pub struct Desktop {
    pub name: String,
    pub comment: String,
    pub exec: String,
    pub icon: String,
    pub terminal: bool,
    pub entry_type: String,
    pub categories: String,
    pub startupnotify: bool,
    pub path: String,
}

impl Default for Desktop {
    fn default() -> Self {
        Self {
            terminal: false,
            entry_type: "Application".to_string(),
            categories: "Application;Game;".to_string(),
            startupnotify: true,
            name: String::default(),
            comment: String::default(),
            exec: String::default(),
            icon: String::default(),
            path: String::default(),
        }
    }
}

impl Desktop {
    pub fn to_string(self) -> String {
        let mut result = String::from("[Desktop Entry]");
        result += format!("\nName={}", &self.name).as_str();
        result += format!("\nComment={}", &self.comment).as_str();
        result += format!("\nExec={}", &self.exec).as_str();
        result += format!("\nIcon={}", self.icon).as_str();
        result += format!("\nTerminal={}", self.terminal.to_string()).as_str();
        result += format!("\nType={}", self.entry_type).as_str();
        result += format!("\nCategories={}", self.categories).as_str();
        result += format!("\nStartupNotify={}", self.startupnotify).as_str();
        result += format!("\nPath={}", self.path).as_str();
        return result;
    }

    pub fn save(self, path: &str) -> anyhow::Result<()> {
        std::fs::write(&path, self.to_string())?;
        Command::new("chmod").args(["+x", &path]).spawn().ok();
        Command::new("update-desktop-database")
            .arg(format!(
                "{}/.local/share/applications",
                env::home_dir().expect("Error").to_string_lossy()
            ))
            .spawn()
            .ok();
        return Ok(());
    }
}
