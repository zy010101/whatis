use std::{fs::File, io::BufReader};
use once_cell::sync::Lazy;
use serde::Deserialize;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    read_config()
});

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rule: RuleConfig
}

#[derive(Debug, Deserialize)]
pub struct RuleConfig {
    pub enable_keywords: bool,
    pub keyword_max_distance_default: u64
}

fn read_config() -> Config {
    let file = File::open("config.yaml").unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(reader).unwrap();

    config
}

