use crate::api_response::ApiResponse;
use crate::database::{self, Ingredient};
use crate::schema::ingredients;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/ingredients")]
pub fn get_ingredients() -> Json<ApiResponse<Vec<Ingredient>>> {
    let conn = database::establish_connection();
    let results = ingredients::table
        .load::<Ingredient>(&conn)
        .expect("Error getting ingredients");

    Json(ApiResponse {
        success: true,
        message: String::from("Ingredients fetched successfully"),
        data: Some(results),
    })
}
