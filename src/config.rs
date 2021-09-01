use serde::Deserialize;
use toml;
use std::fs;
use once_cell::sync::Lazy;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub shared_dir: String,
    pub base_dir: String
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let raw = {
        let mut config = env::current_dir().expect("fail to get current directory");
        config.push("config");

        let mut app_config = config.clone();
        let mut local_config = config.clone();

        app_config.push("app_config.toml");
        local_config.push("local_config.toml");

        let app_config = fs::read_to_string(&app_config).expect(
            format!("file not found: {:?}", &app_config).as_str()
        );
        let local_config = fs::read_to_string(&local_config).expect(
            format!("file not found: {:?}", &local_config).as_str()
        );

        app_config + "\n" + &local_config
    };
    toml::from_str(&raw).expect("fail to parse toml")
});
