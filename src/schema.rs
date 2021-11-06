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
