use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use axum::extract::State;

use super::schema::AppSchema;

pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
