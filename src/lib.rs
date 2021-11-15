#[macro_use]
extern crate diesel;

pub mod database;
pub mod schema;

use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use schema::ingredients;

#[derive(Serialize, Deserialize, Debug, DbEnum)]
pub enum IngredientCategory {
    Burger,
    Topping,
    Bun,
    Sauce,
    SideDish,
    Drink,
}

#[derive(Insertable, Deserialize, AsChangeset, Debug)]
pub struct Ingredient {
    pub name: String,
    pub amount: i32,
    pub category: IngredientCategory,
    pub price: f32,
}

impl Ingredient {
    pub fn set(items: Vec<Ingredient>) -> QueryResult<usize> {
        let conn = database::establish_connection();

        diesel::insert_into(ingredients::table)
            .values(&items)
            .execute(&conn)
    }
    pub fn get() -> QueryResult<Vec<database::Ingredient>> {
        let conn = database::establish_connection();

        ingredients::table
            .order(ingredients::category.asc())
            .load(&conn)
    }
    pub fn update(id: u32, new: Ingredient) -> QueryResult<usize> {
        let id = id as i32;
        let conn = database::establish_connection();

        let target = ingredients::table.filter(ingredients::id.eq(&id));

        diesel::update(target).set(new).execute(&conn)
    }
    pub fn delete(id: u32) -> QueryResult<usize> {
        let id = id as i32;

        let conn = database::establish_connection();

        let target = ingredients::table.filter(ingredients::id.eq(&id));

        diesel::delete(target).execute(&conn)
    }
}

#[derive(DbEnum, Debug)]
enum TableStatus {
    Available,
    InUse,
}

#[derive(DbEnum, Debug)]
enum OrderStatus {
    NotPaid,
    InQueue,
    Processing,
    Fulfilled,
}
