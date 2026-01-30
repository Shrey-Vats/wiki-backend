use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoValidationError {
    #[error("Todo must be 5 cherecters long")]
    TodoTooShort,
    #[error("Description must be 5 cherecters long")]
    DescriptionTooShort,
    #[error("Tag must be 2 cherecter long or alphabets only")]
    InvalidTag,
    #[error("Category must be 2 cherecter long or alphabet only")]
    InvalidCategories
}