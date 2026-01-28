use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub message: &'static str,
    pub success: bool,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: &'static str, data: T)-> Self {
        Self {message, data: Some(data), success: true }
    }
}

