use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigModel {
    pub app: AppConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub host: String,
    pub port: u16,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
}
