use crate::schema::ingredients;
use crate::{database, IngredientCategory};
use diesel::prelude::*;

#[derive(Queryable, Debug)]
struct Ingredient {
    id: i32,
    name: String,
    amount: i32,
    category: IngredientCategory,
    price: f32,
}

#[get("/ingredients")]
pub fn get_ingredients() {
    let conn = database::establish_connection();
    let results = ingredients::table
        .load::<Ingredient>(&conn)
        .expect("Error getting ingredients");
    println!("{:#?}", results);
}
