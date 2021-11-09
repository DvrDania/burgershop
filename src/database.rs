use crate::IngredientCategory;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

/// establish connection to Postgres database
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// schema
table! {
    use diesel::sql_types::*;
    use crate::IngredientCategoryMapping;

    ingredients (id) {
        id -> Int4,
        name -> Varchar,
        amount -> Int4,
        category -> IngredientCategoryMapping,
        price -> Float4,
    }
}

/// database ingredients structure
#[derive(Queryable, Debug)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub category: IngredientCategory,
    pub price: f32,
}
