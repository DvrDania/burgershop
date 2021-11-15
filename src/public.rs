use crate::api_response::ApiResponse;
use rocket::serde::json::Json;

#[get("/ingredients")]
pub fn get_ingredients() -> Json<ApiResponse<Vec<burgershop::database::Ingredient>>> {
    match burgershop::Ingredient::get() {
        Ok(ingredients) => Json(ApiResponse {
            success: true,
            message: format!("Got {} ingredients", ingredients.len()),
            data: Some(ingredients),
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            message: e.to_string(),
            data: None,
        }),
    }
}
