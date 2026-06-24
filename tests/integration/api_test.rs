use super::*;
use axum::http::StatusCode;

#[tokio::test]
async fn test_get_users() {
    let app = Router::new().route("/users", get(get_users));
    let response = app.clone().oneshot(Request::get("/users").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_user_api() {
    let app = Router::new().route("/users", post(create_user_handler));
    let new_user = CreateUser {
        email: "newuser@example.com".to_string(),
        hashed_password: "securepassword".to_string(),
    };
    let response = app.clone().oneshot(Request::post("/users").json(&new_user).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}