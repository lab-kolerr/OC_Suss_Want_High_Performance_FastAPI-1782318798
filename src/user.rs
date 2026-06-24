use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hashed_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}