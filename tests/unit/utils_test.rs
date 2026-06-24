use super::*;

#[test]
fn test_hash_password() {
    let password = "mysecretpassword";
    let hashed = hash_password(password);
    assert_ne!(hashed, password);
}

#[test]
fn test_verify_password() {
    let password = "mysecretpassword";
    let hashed = hash_password(password);
    assert!(verify_password(password, &hashed));
}