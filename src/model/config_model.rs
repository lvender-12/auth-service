use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub app: AppConfig,
    pub db: DbConfig,
    pub jwt: JwtConfig,
    pub api: ApiConfig,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry: u64,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub secret: String,
}
