use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleResponseDto {
    pub id: i32,
    pub name: String,
}
