use async_graphql::{EmptySubscription, Schema};
use axum::{Router, routing::post};

use crate::{
    graphql::{
        handler::graphql_handler, mutation::MutationRoot, query::QueryRoot, schema::AppSchema,
    },
    models::state_model::AppState,
};

pub fn guest_router(state: AppState) -> Router<AppState> {
    let schema: AppSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(state.clone())
        .finish();

    Router::new()
        .route("/graphql", post(graphql_handler))
        .with_state(schema)
}
