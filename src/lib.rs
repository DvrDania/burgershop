#[macro_use]
extern crate diesel;

pub mod database;
pub mod schema;

use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use schema::{ingredients, tables};

#[derive(Serialize, Deserialize, Debug, DbEnum, Clone)]
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
pub enum TableStatus {
    Available,
    InUse,
}

#[derive(Insertable)]
#[table_name = "tables"]
pub struct BShopTable {
    number: i32,
    status: TableStatus,
}

impl BShopTable {
    pub fn from_numbers(numbers: Vec<u32>) -> QueryResult<usize> {
        let conn = database::establish_connection();
        let mut tables_vec = vec![];

        for number in numbers {
            let number = number as i32;
            let new_table = BShopTable {
                number,
                status: TableStatus::Available,
            };
            tables_vec.push(new_table);
        }
        diesel::insert_into(tables::table)
            .values(tables_vec)
            .execute(&conn)
    }
}

#[derive(DbEnum, Debug)]
enum OrderStatus {
    NotPaid,
    InQueue,
    Processing,
    Fulfilled,
}
