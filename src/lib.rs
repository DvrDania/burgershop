#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

pub mod database;
pub mod models;
pub mod moderator;
pub mod schema;

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
