use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserValidationError {
    #[error("user enter invalid email")]
    InvalidEmail,
    #[error("Invalid Password")]
    InvalidPassword,
    #[error("Name must be 3 cherecter long")]
    TooShortName,
    #[error("User already exits")]
    UserAlreadyExits,
    #[error("Failed to create token")]
    FailedToCreateToken
}


