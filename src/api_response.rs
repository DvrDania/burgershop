use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn from(data: Option<T>, message: String) -> Json<Self> {
        Json(ApiResponse {
            success: true,
            message,
            data,
        })
    }

    pub fn from_error(e: diesel::result::Error) -> Json<Self> {
        Json(ApiResponse {
            success: false,
            message: e.to_string(),
            data: None,
        })
    }
}
