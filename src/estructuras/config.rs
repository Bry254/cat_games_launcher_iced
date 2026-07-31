use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub global: HashMap<String, RunnerOption>,
    pub runners: HashMap<String, Runner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Runner {
    #[serde(default)]
    pub command: String,
    pub options: HashMap<String, RunnerOption>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct RunnerOption {
    #[serde(rename = "type")]
    pub kind: String,
    pub cmd: String,
    pub mode: String,
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub input: String,
    #[serde(default)]
    pub values: Vec<String>,
    #[serde(default)]
    pub var: String,
}
