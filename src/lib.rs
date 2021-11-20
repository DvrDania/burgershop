#[macro_use]
extern crate diesel;

pub mod database;
pub mod schema;

use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use schema::{ingredients, order_item_ingredients, order_items, orders, tables};

#[derive(Serialize, Deserialize, Debug, DbEnum, Clone)]
pub enum IngredientCategory {
    Bun,
    Burger,
    Topping,
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
    pub fn set(numbers: Vec<u32>) -> QueryResult<usize> {
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
    pub fn take(table_number: u32) -> QueryResult<usize> {
        let conn = database::establish_connection();

        diesel::update(tables::table.find(table_number as i32))
            .set(tables::status.eq(TableStatus::InUse))
            .execute(&conn)
    }
    pub fn leave(table_number: u32) -> QueryResult<usize> {
        let conn = database::establish_connection();

        diesel::update(tables::table.find(table_number as i32))
            .set(tables::status.eq(TableStatus::Available))
            .execute(&conn)
    }
}

#[derive(DbEnum, Debug)]
pub enum OrderStatus {
    NotPaid,
    InQueue,
    Processing,
    Fulfilled,
}

#[derive(Insertable)]
pub struct Order {
    pub table_number: i32,
    pub status: OrderStatus,
    pub total: f32,
}
impl Order {
    /// creates new order for specified table
    pub fn new(table_number: u32) -> QueryResult<usize> {
        let table_number = table_number as i32;
        let conn = database::establish_connection();

        diesel::insert_into(orders::table)
            .values(Order {
                table_number,
                status: OrderStatus::NotPaid,
                total: 0.0,
            })
            .execute(&conn)
    }
    pub fn add_ingredient() {}
}

#[derive(Insertable)]
pub struct OrderItem {
    pub order_id: i32,
    pub total: f32,
}
impl OrderItem {
    /// adds new item for specified order
    pub fn new(order_id: u32) -> QueryResult<usize> {
        let order_id = order_id as i32;
        let conn = database::establish_connection();

        diesel::insert_into(order_items::table)
            .values(OrderItem {
                order_id,
                total: 0.0,
            })
            .execute(&conn)
    }
}

#[derive(Insertable)]
pub struct OrderItemIngredient {
    pub order_item_id: i32,
    pub ingredient_id: i32,
    pub amount: i32,
    pub total: f32,
}
impl OrderItemIngredient {
    pub fn add(order_item_id: u32, ingredient_id: u32, amount: u32) -> QueryResult<usize> {
        let conn = database::establish_connection();

        let total = ingredients::table
            .find(ingredient_id as i32)
            .first::<database::Ingredient>(&conn)?
            .price
            * amount as f32;

        let order_item_id = order_item_id as i32;
        let ingredient_id = ingredient_id as i32;
        let amount = amount as i32;

        let new = OrderItemIngredient {
            order_item_id,
            ingredient_id,
            amount,
            total,
        };

        diesel::insert_into(order_item_ingredients::table)
            .values(new)
            .execute(&conn)
    }
}
