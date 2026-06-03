use async_graphql::{Context, Object, Result};

use crate::{
    dto::user_dto::{CreateUserDto, LoginUserDto},
    models::state_model::AppState,
    services::user_service::{login_user, register_user},
};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn register(
        &self,
        ctx: &Context<'_>,
        nim: String,
        name: String,
        email: Option<String>,
        password: String,
    ) -> Result<bool> {
        let state = ctx.data::<AppState>()?;

        register_user(
            state,
            CreateUserDto {
                nim,
                name,
                email,
                password,
            },
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(true)
    }

    async fn login(&self, ctx: &Context<'_>, nim: String, password: String) -> Result<String> {
        let state = ctx.data::<AppState>()?;

        let result = login_user(state, LoginUserDto { nim, password })
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(result.token)
    }
}
