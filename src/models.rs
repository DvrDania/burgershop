use crate::IngredientCategory;
use diesel::Queryable;

#[derive(Queryable)]
pub struct Ingredient {
    pub name: String,
    pub amount: u32,
    pub category: IngredientCategory,
    pub price: f32,
}
