use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32, // user id
    pub nim: String,
    pub role_id: i32,
    pub exp: usize, // expired time
}
