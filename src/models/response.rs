// src/models/response.rs

use serde::Serialize;
use schemars::JsonSchema;

#[derive(Serialize, JsonSchema)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
    pub code: u16,
}
