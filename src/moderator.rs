use crate::api_response::ApiResponse;
use crate::database;
use crate::schema::ingredients;
use diesel::prelude::*;
use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;

#[post("/ingredients", data = "<items>", format = "json")]
pub fn set_ingredients(
    items: Json<Vec<crate::Ingredient>>,
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

#[put("/ingredients/<id>", data = "<new_value>", format = "json")]
pub fn update_ingredient(
    id: u32,
    new_value: Json<crate::Ingredient>,
) -> Json<ApiResponse<database::Ingredient>> {
    let id = id as i32;
    let new_value = new_value.into_inner();

    let conn = database::establish_connection();

    let target = ingredients::table.filter(ingredients::id.eq(&id));

    match diesel::update(target)
        .set(new_value)
        .get_result::<database::Ingredient>(&conn)
    {
        Ok(ingredient) => Json(ApiResponse {
            success: true,
            message: String::from("Ingredient updated successfully"),
            data: Some(ingredient),
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            message: e.to_string(),
            data: None,
        }),
    }
}

#[delete("/ingredients/<id>")]
pub fn delete_ingredient(id: u32) {
    let id = id as i32;
    let conn = database::establish_connection();
    let result = diesel::delete(ingredients::table.filter(ingredients::id.eq(&id))).execute(&conn);
    println!("{:#?}", result);
}
