use crate::{IngredientCategory, TableStatus};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::serde::Serialize;
use std::env;

/// establish connection to Postgres database
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

/// database ingredients structure
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub category: IngredientCategory,
    pub price: f32,
}

#[derive(Queryable)]
pub struct BShopTable {
    pub id: i32,
    pub number: i32,
    pub status: TableStatus,
}
