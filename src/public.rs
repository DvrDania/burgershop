use crate::database::{self, ingredients, Ingredient};
use diesel::prelude::*;

#[get("/ingredients")]
pub fn get_ingredients() {
    let conn = database::establish_connection();
    let results = ingredients::table
        .load::<Ingredient>(&conn)
        .expect("Error getting ingredients");
    println!("{:#?}", results);
}
