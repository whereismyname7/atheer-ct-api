use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}
