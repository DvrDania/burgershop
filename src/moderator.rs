use rocket::serde::json::Json;

use crate::api_response::ApiResponse;

#[post("/ingredients", data = "<items>", format = "json")]
pub fn set_ingredients(items: Json<Vec<burgershop::Ingredient>>) -> Json<ApiResponse<()>> {
    let items = items.into_inner();
    match burgershop::Ingredient::set(items) {
        Ok(_) => Json(ApiResponse {
            success: true,
            message: "Ingredients set successfully".to_string(),
            data: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            message: e.to_string(),
            data: None,
        }),
    }
}

#[put("/ingredients/<id>", data = "<new_value>", format = "json")]
pub fn update_ingredient(
    id: u32,
    new_value: Json<burgershop::Ingredient>,
) -> Json<ApiResponse<()>> {
    let new_value = new_value.into_inner();
    match burgershop::Ingredient::update(id, new_value) {
        Ok(_) => Json(ApiResponse {
            success: true,
            message: "Ingredient updated successfully".to_string(),
            data: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            message: e.to_string(),
            data: None,
        }),
    }
}

#[delete("/ingredients/<id>")]
pub fn delete_ingredient(id: u32) -> Json<ApiResponse<()>> {
    match burgershop::Ingredient::delete(id) {
        Ok(_) => Json(ApiResponse {
            success: true,
            message: "Ingredient deleted successfully".to_string(),
            data: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            message: e.to_string(),
            data: None,
        }),
    }
}
