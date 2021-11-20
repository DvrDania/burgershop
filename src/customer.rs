use crate::api_response::ApiResponse;
use rocket::serde::json::Json;

#[get("/ingredients")]
pub fn get_ingredients() -> Json<ApiResponse<Vec<burgershop::database::Ingredient>>> {
    match burgershop::Ingredient::get() {
        Ok(burgers) => ApiResponse::from(
            Some(burgers.clone()),
            format!("No. of results: {}", burgers.len()),
        ),
        Err(e) => ApiResponse::from_error(e),
    }
}
