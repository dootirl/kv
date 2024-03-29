use std::env;
use std::fs;

use serde::Deserialize;

use crate::utils::exit_with_msg;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_listen_address")]
    pub listen_address: String,

    pub cert_path: String,
    pub key_path: String,

    #[serde(default = "default_store_url")]
    pub store_url: String,
}

// Serde defaults don't support string literals
fn default_listen_address() -> String {
    "127.0.0.1:3000".to_string()
}

fn default_store_url() -> String {
    "https://[::1]:50051".to_string()
}

impl Config {
    pub fn build() -> Config {
        let filename = env::var("CONFIG_PATH").unwrap_or("config.toml".to_string());

        let contents = fs::read_to_string(&filename).unwrap_or_else(|_| {
            exit_with_msg(format!("Could not read file `{filename}`").as_str())
        });

        toml::from_str(&contents).unwrap_or_else(|e| exit_with_msg(format!("{e}").as_str()))
    }
}
