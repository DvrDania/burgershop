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

table! {
    order_items (id) {
        id -> Int4,
        ingredient_id -> Int4,
        order_id -> Int4,
        amount -> Int4,
        total -> Float4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::OrderStatusMapping;
    orders (id) {
        id -> Int4,
        table_id -> Int4,
        status -> OrderStatusMapping,
        total -> Float4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::TableStatusMapping;
    tables (id) {
        id -> Int4,
        number -> Int4,
        status -> TableStatusMapping,
    }
}

joinable!(order_items -> ingredients (ingredient_id));
joinable!(order_items -> orders (order_id));
joinable!(orders -> tables (table_id));

allow_tables_to_appear_in_same_query!(ingredients, order_items, orders, tables,);
