use std::{fs::File, io::Read};

use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    pub executable: Option<String>,
    pub mods: Option<Vec<String>>,
}

impl Config {
    pub fn get_config() -> Self {
        if let Ok(mut file) = File::open("modconfig.toml") {
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();

            if let Ok(config) = toml::from_str(&data) {
                config
            } else {
                Default::default()
            }
        } else {
            Default::default()
        }
    }
}
