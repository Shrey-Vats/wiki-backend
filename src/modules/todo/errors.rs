use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoValidationError {
    #[error("Todo must be 5 cherecters long")]
    TodoTooShort,
    #[error("Description must be 5 cherecters long")]
    DescriptionTooShort
}