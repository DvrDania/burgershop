use crate::api_response::ApiResponse;
use crate::database::{self, ingredients};
use crate::Ingredient;
use diesel::prelude::*;
use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;

#[post("/ingredients", data = "<items>", format = "json")]
pub fn set_ingredients(
    items: Json<Vec<Ingredient>>,
) -> (Status, (ContentType, Json<ApiResponse<()>>)) {
    let conn = database::establish_connection();
    let items = items.into_inner();

    diesel::insert_into(ingredients::table)
        .values(&items)
        .execute(&conn)
        .expect("Error saving ingredients");
    let res: ApiResponse<()> = ApiResponse {
        success: true,
        message: String::from("Ingredients set successfully"),
        data: None,
    };
    (Status::Created, (ContentType::JSON, Json(res)))
}
