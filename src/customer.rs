use crate::api_response::ApiResponse;
use burgershop::IngredientCategory;
use rocket::serde::json::Json;

#[get("/ingredients/burgers")]
pub fn get_burgers() -> Json<ApiResponse<Vec<burgershop::database::Ingredient>>> {
    match burgershop::Ingredient::get(Some(IngredientCategory::Burger)) {
        Ok(burgers) => ApiResponse::from(
            Some(burgers.clone()),
            format!("Got {} burgers", burgers.len()),
        ),
        Err(e) => ApiResponse::from_error(e),
    }
}
