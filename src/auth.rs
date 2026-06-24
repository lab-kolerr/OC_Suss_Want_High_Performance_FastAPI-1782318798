use axum::{Json, Extension};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User not found")] 
    UserNotFound,
    #[error("Invalid password")] 
    InvalidPassword,
    #[error("Token error")] 
    TokenError,
}

pub async fn register(Json(payload): Json<CreateUser>, Extension(pool): Extension<PgPool>) -> Result<Json<User>, AuthError> {
    payload.validate().map_err(|_| AuthError::InvalidPassword)?;
    let hashed_password = hash(&payload.password, 12).map_err(|_| AuthError::TokenError)?;
    let user = User { id: 0, email: payload.email, hashed_password, role: "user".to_string() };
    sqlx::query("INSERT INTO users (email, hashed_password, role) VALUES ($1, $2, $3)")
        .bind(&user.email)
        .bind(&user.hashed_password)
        .bind(&user.role)
        .execute(&pool).await.map_err(|_| AuthError::TokenError)?;
    Ok(Json(user))
}

pub async fn login(Json(payload): Json<LoginRequest>, Extension(pool): Extension<PgPool>) -> Result<Json<AuthResponse>, AuthError> {
    let user: User = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_one(&pool).await.map_err(|_| AuthError::UserNotFound)?;
    if !verify(&payload.password, &user.hashed_password).map_err(|_| AuthError::TokenError)? {
        return Err(AuthError::InvalidPassword);
    }
    let access_token = encode(&Header::default(), &user, &"secret_access".as_bytes()).map_err(|_| AuthError::TokenError)?;
    let refresh_token = encode(&Header::default(), &user, &"secret_refresh".as_bytes()).map_err(|_| AuthError::TokenError)?;
    Ok(Json(AuthResponse { access_token, refresh_token }))
}