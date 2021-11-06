use crate::schema::ingredients;
use crate::IngredientCategory;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "ingredients"]
pub struct Ingredient {
    pub name: String,
    pub amount: i32,
    pub category: IngredientCategory,
    pub price: f32,
}
