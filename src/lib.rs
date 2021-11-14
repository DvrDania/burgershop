#[macro_use]
extern crate diesel;

pub mod database;
pub mod schema;

use diesel::prelude::*;
use diesel::ConnectionError;
use diesel_derive_enum::DbEnum;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use schema::ingredients;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, DbEnum)]
pub enum IngredientCategory {
    Burger,
    Topping,
    Bun,
    Sauce,
    SideDish,
    Drink,
}

#[derive(Insertable, Deserialize, AsChangeset)]
pub struct Ingredient {
    pub name: String,
    pub amount: i32,
    pub category: IngredientCategory,
    pub price: f32,
}

pub struct TestType {}

impl Ingredient {
    pub fn set(items: Vec<Ingredient>) -> Result<(), Box<dyn Error>> {
        let conn = database::establish_connection()?;

        diesel::insert_into(ingredients::table)
            .values(&items)
            .execute(&conn)?;

        Ok(())
    }
    pub fn get() -> Result<Vec<database::Ingredient>, Box<dyn Error>> {
        let conn = database::establish_connection()?;

        let results = ingredients::table
            .order(ingredients::category.asc())
            .load(&conn)?;
        Ok(results)
    }
    pub fn update(id: u32, new: Ingredient) -> Result<(), Box<dyn Error>> {
        let id = id as i32;
        let conn = database::establish_connection()?;

        let target = ingredients::table.filter(ingredients::id.eq(&id));

        diesel::update(target).set(new).execute(&conn)?;
        Ok(())
    }
    pub fn delete(id: u32) -> Result<(), Box<dyn Error>> {
        let id = id as i32;

        let conn = database::establish_connection()?;

        let target = ingredients::table.filter(ingredients::id.eq(&id));

        diesel::delete(target).execute(&conn)?;
        Ok(())
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
