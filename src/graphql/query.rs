use async_graphql::{Context, Object};

use crate::models::state_model::AppState;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self) -> &str {
        "pong"
    }

    async fn profile(&self, ctx: &Context<'_>) -> String {
        let state = ctx.data::<AppState>().unwrap();

        format!("connected: {}", state.config.app.host)
    }
}
