use serde::Deserialize;
use validator::Validate;
use validator::ValidationError;

#[derive(Deserialize, Validate)]
pub struct User {
    #[validate(email)]
    pub email: String,

    #[validate(custom = "validate_password")]
    pub password: String,
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| "@$!%*?&".contains(c));
    let long_enough = password.len() >= 8;

    if has_upper && has_lower && has_digit && has_special && long_enough {
        Ok(())
    } else {
        Err(ValidationError::new("password"))
    }
}
