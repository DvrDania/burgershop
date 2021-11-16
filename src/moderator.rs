use rocket::serde::json::Json;

use crate::api_response::ApiResponse;

#[post("/ingredients", data = "<items>", format = "json")]
pub fn set_ingredients(items: Json<Vec<burgershop::Ingredient>>) -> Json<ApiResponse<()>> {
    let items = items.into_inner();
    match burgershop::Ingredient::set(items) {
        Ok(_) => ApiResponse::from(None, "Ingredients set successfully".to_string()),
        Err(e) => ApiResponse::from_error(e),
    }
}

#[put("/ingredients/<id>", data = "<new_value>", format = "json")]
pub fn update_ingredient(
    id: u32,
    new_value: Json<burgershop::Ingredient>,
) -> Json<ApiResponse<()>> {
    let new_value = new_value.into_inner();
    match burgershop::Ingredient::update(id, new_value) {
        Ok(_) => ApiResponse::from(None, "Ingredient updated successfully".to_string()),
        Err(e) => ApiResponse::from_error(e),
    }
}

#[delete("/ingredients/<id>")]
pub fn delete_ingredient(id: u32) -> Json<ApiResponse<()>> {
    match burgershop::Ingredient::delete(id) {
        Ok(_) => ApiResponse::from(None, "Ingredient deleted successfully".to_string()),
        Err(e) => ApiResponse::from_error(e),
    }
}

#[post("/tables", format = "json", data = "<numbers>")]
pub fn set_tables(numbers: Json<Vec<u32>>) -> Json<ApiResponse<()>> {
    let numbers = numbers.into_inner();
    match burgershop::BShopTable::from_numbers(numbers) {
        Ok(_) => ApiResponse::from(None, "Tables set successfully".to_string()),
        Err(e) => ApiResponse::from_error(e),
    }
}
