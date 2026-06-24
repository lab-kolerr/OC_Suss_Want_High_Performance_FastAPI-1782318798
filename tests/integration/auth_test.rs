use super::*;
use axum::http::StatusCode;

#[tokio::test]
async fn test_login_success() {
    let app = Router::new().route("/login", post(login_handler));
    let login_data = LoginRequest {
        email: "test@example.com".to_string(),
        password: "securepassword".to_string(),
    };
    let response = app.clone().oneshot(Request::post("/login").json(&login_data).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_login_failure() {
    let app = Router::new().route("/login", post(login_handler));
    let login_data = LoginRequest {
        email: "wrong@example.com".to_string(),
        password: "wrongpassword".to_string(),
    };
    let response = app.clone().oneshot(Request::post("/login").json(&login_data).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}