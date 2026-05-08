use anyhow::Result;
use config::{Config, File, FileFormat};

use crate::models::config_model::ConfigModel;

pub async fn load_config() -> Result<ConfigModel> {
    let config: ConfigModel = Config::builder()
        .add_source(File::new("config.yaml", FileFormat::Yaml))
        .build()
        .unwrap()
        .try_deserialize()?;

    return Ok(config);
}
