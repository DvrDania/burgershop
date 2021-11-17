use crate::api_response::ApiResponse;
use burgershop::{IngredientCategory, Order};
use diesel::result::Error::NotFound;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;

#[get("/ingredients/burgers")]
pub fn get_burgers() -> Json<ApiResponse<Vec<burgershop::database::Ingredient>>> {
    match burgershop::Ingredient::get(Some(IngredientCategory::Burger)) {
        Ok(burgers) => ApiResponse::from(
            Some(burgers.clone()),
            format!("No. of results: {}", burgers.len()),
        ),
        Err(e) => ApiResponse::from_error(e),
    }
}

#[derive(Deserialize)]
pub struct OrderItemIngredient {
    pub id: u32,
    pub amount: u32,
}
#[derive(Deserialize)]
pub struct OrderItem {
    pub table_number: u32,
    pub ingredients: Vec<OrderItemIngredient>,
}

#[post("/ingredients/burgers", format = "json", data = "<data>")]
pub fn set_burgers(data: Json<OrderItem>) -> Json<ApiResponse<()>> {
    let data = data.into_inner();
    let new_order_id = match Order::new(data.table_number) {
        Ok(id) => id,
        Err(e) => match e {
            NotFound => return ApiResponse::from_error_str("Couldn't find table"),
            _ => return ApiResponse::from_error(e),
        },
    };
    for ingredient in data.ingredients {
        match Order::add_item(new_order_id as u32, ingredient.id, ingredient.amount) {
            Ok(_) => continue,
            Err(e) => return ApiResponse::from_error(e),
        };
    }
    ApiResponse::from(None, "Burgers added to order".to_string())
}
