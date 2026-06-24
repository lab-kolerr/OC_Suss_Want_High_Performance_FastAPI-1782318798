use super::*;
use validator::Validate;

#[test]
fn test_user_validation() {
    let user = CreateUser {
        email: "test@example.com".to_string(),
        hashed_password: "securepassword".to_string(),
    };
    assert!(user.validate().is_ok());
}

#[test]
fn test_user_invalid_email() {
    let user = CreateUser {
        email: "invalid-email".to_string(),
        hashed_password: "securepassword".to_string(),
    };
    assert!(user.validate().is_err());
}