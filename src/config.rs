use anyhow::{Context, Result};
use serde::Deserialize;
use serde_yaml;
use std::{collections::BTreeMap, fs::File};

#[derive(Deserialize, Debug)]
pub struct Host {
    pub port: u16,
    pub dest: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "ssh_config_default")]
    pub ssh_config: String,
    pub hosts: BTreeMap<String, ()>,
}

fn ssh_config_default() -> String {
    "/etc/killswitch/ssh/config".to_string()
}

pub fn get_config(path: &str) -> Result<Config> {
    let f = File::open(path).with_context(|| format!("Couldn't open config file {}", path))?;
    Ok(serde_yaml::from_reader(f)?)
}
