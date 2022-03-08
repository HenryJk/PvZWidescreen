use std::{fs::File, io::Read};

use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    pub visibility: Option<String>,
    pub color: Option<String>,
    pub plant_hb_visible: Option<bool>,
    pub zombie_hb_visible: Option<bool>,
}

impl Config {
    pub fn get_config() -> Self {
        if let Ok(mut file) = File::open("healthbar_config.toml") {
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
