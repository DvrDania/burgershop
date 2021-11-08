use crate::database;
use crate::schema::ingredients;
use crate::Ingredient;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[post("/ingredients", data = "<items>", format = "json")]
pub fn fill_ingredients(items: Json<Vec<Ingredient>>) {
    let conn = database::establish_connection();
    let items = items.into_inner();

    diesel::insert_into(ingredients::table)
        .values(&items)
        .execute(&conn)
        .expect("Error saving ingredients");
}
