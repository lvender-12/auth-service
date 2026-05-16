use config::{Config, File, FileFormat};

use crate::model::config_model::Settings;

pub fn load_config() -> Settings {
    let settings: Settings = Config::builder()
        .add_source(File::new("config/config.yaml", FileFormat::Yaml))
        .build()
        .expect("Can't find config.yaml")
        .try_deserialize()
        .expect("can't deserialize");
    settings
}
