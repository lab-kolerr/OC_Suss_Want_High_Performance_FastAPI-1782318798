use super::*;

#[test]
fn test_create_user_service() {
    let user = CreateUser {
        email: "newuser@example.com".to_string(),
        hashed_password: "securepassword".to_string(),
    };
    let result = create_user(user);
    assert!(result.is_ok());
}

#[test]
fn test_create_user_service_invalid() {
    let user = CreateUser {
        email: "invalid-email".to_string(),
        hashed_password: "short".to_string(),
    };
    let result = create_user(user);
    assert!(result.is_err());
}