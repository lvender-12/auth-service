use config::{Config, File, FileFormat};

use crate::models::config_model::ConfigModel;

pub async fn load_config() -> ConfigModel {
    let config: ConfigModel = Config::builder()
        .add_source(File::new("config.yaml", FileFormat::Yaml))
        .build()
        .expect("didn't found file config.yaml")
        .try_deserialize()
        .expect("cannot deserialize config.yaml");

    return config;
}
