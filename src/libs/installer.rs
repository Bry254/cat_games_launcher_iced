use std::{
    collections::HashMap,
    fs::{self, File},
    io::copy,
    path::Path,
    process::Command,
};

use crate::{
    estructuras::config::{Config, Runner, RunnerOption},
    libs::{
        dialogos,
        extractor::{self},
        vars::{self},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    mode: String,
    data: String,
    #[serde(default)]
    target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Runnerinstaller {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub command: String,
    pub options: HashMap<String, RunnerOption>,
    pub actions: Vec<Action>,
}

impl Runnerinstaller {
    pub fn new(path: &String) -> anyhow::Result<Self> {
        let data = fs::read_to_string(path)?;
        println!("{data}");
        let r: Self = serde_json::from_str(data.as_str())?;
        println!("{:?}", r);
        return Ok(r);
    }

    pub fn import(self, config: &mut Config) -> anyhow::Result<()> {
        let run = Runner {
            command: self.command,
            options: self.options,
        };
        config.runners.insert(self.name.clone(), run);
        if let Ok(data) = serde_json::to_string_pretty(&config) {
            fs::write(format!("{}/options.json", vars::Variables::CONFIG()), data)?
        };
        Ok(())
    }

    pub fn install(&self) -> anyhow::Result<()> {
        if !dialogos::confirm(
            format!("Instalar {} ?", self.name).as_str(),
            &self.description,
        ) {
            return Ok(());
        }
        let variables = vars::Variables::default();
        for i in &self.actions {
            let data = variables.apply(&i.data);
            let target = variables.apply(&i.target);
            match i.mode.as_str() {
                "download" => {
                    println!("Descargando archivo {data} ....");
                    download_file(&data, &target)?;
                }
                "zip" => {
                    println!("Extrayendo {data} en {target}");
                    extractor::unzip(&data, &target)?;
                }
                "tar" => {
                    println!("Extrayendo {data} en {target}");
                    extractor::tar(&data, &target)?;
                }
                "bash" => {
                    println!("Ejecutando comando en bash... {data}");
                    Command::new("bash").args(["-c", data.as_str()]).spawn()?;
                }
                _ => return Err(anyhow::anyhow!("Accion desconcida")),
            };
        }
        println!("Instalación finalizada");
        return Ok(());
    }
}

pub fn download_file(url: &String, path: &String) -> anyhow::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?; // Crea la carpeta si no existe
    }
    let mut file = File::create(path)?;
    let mut resp = ureq::get(url).call()?;
    copy(&mut resp.body_mut().as_reader(), &mut file)?;
    Ok(())
}

//vars $localdata $home $tmp
// mode:download data:url target:path
// mode:extractzip data:file target:path
// mode:extracttar data:file target:path
// mode:rm data:file
