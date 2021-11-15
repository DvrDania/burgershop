use crate::api_response::ApiResponse;
use rocket::serde::json::Json;

#[get("/ingredients")]
pub fn get_ingredients() -> Json<ApiResponse<Vec<burgershop::database::Ingredient>>> {
    match burgershop::Ingredient::get() {
        Ok(ingredients) => ApiResponse::from(
            Some(ingredients.clone()),
            format!("Got {} ingredients", ingredients.len()),
        ),
        Err(e) => ApiResponse::from_error(e),
    }
}
