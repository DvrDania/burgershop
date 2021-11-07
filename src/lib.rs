#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

pub mod database;
pub mod moderator;
pub mod public;
pub mod schema;

use crate::schema::ingredients;
use diesel_derive_enum::DbEnum;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, DbEnum)]
pub enum IngredientCategory {
    Burger,
    Topping,
    Bread,
    Sauce,
    SideDish,
}

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "ingredients"]
pub struct Ingredient {
    pub name: String,
    pub amount: i32,
    pub category: IngredientCategory,
    pub price: f32,
}
