use std::sync::Arc;

use crate::models::config_model::ConfigModel;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Arc<ConfigModel>,
}
