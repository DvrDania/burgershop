#[macro_use]
extern crate diesel;

pub mod database;
pub mod schema;

use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use schema::{ingredients, order_items, orders, tables};

#[derive(Serialize, Deserialize, Debug, DbEnum, Clone)]
pub enum IngredientCategory {
    Burger,
    Topping,
    Bun,
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
    pub fn get(category: Option<IngredientCategory>) -> QueryResult<Vec<database::Ingredient>> {
        let conn = database::establish_connection();

        match category {
            Some(category) => ingredients::table
                .filter(ingredients::category.eq(category))
                .order(ingredients::category.asc())
                .load(&conn),
            None => ingredients::table
                .order(ingredients::category.asc())
                .load(&conn),
        }
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
    pub fn from_numbers(numbers: Vec<u32>) -> QueryResult<usize> {
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
    pub table_id: i32,
    pub status: OrderStatus,
    pub total: f32,
}

impl Order {
    pub fn add_item(order_id: u32, ingredient_id: u32, amount: u32) -> QueryResult<usize> {
        let order_id = order_id as i32;
        let ingredient_id = ingredient_id as i32;
        let amount = amount as i32;

        let conn = database::establish_connection();

        let order_item = OrderItem {
            ingredient_id,
            order_id,
            amount,
            total: OrderItem::calculate_total(ingredient_id as u32, amount as u32)?,
        };

        diesel::update(orders::table.filter(orders::id.eq(order_item.order_id)))
            .set(orders::total.eq(orders::total + order_item.total))
            .execute(&conn)?;

        diesel::insert_into(order_items::table)
            .values(&order_item)
            .execute(&conn)
    }
    /// creates a new order and returns its id
    pub fn new(table_number: u32) -> QueryResult<i32> {
        let conn = database::establish_connection();
        let table_number = table_number as i32;
        let table_id = tables::table
            .filter(tables::number.eq(table_number))
            .first::<database::BShopTable>(&conn)?
            .id;

        let new_order = Order {
            table_id,
            status: OrderStatus::NotPaid,
            total: 0.0,
        };

        Ok(diesel::insert_into(orders::table)
            .values(&new_order)
            .get_result::<database::Order>(&conn)?
            .id)
    }
}

#[derive(Insertable)]
pub struct OrderItem {
    pub ingredient_id: i32,
    pub order_id: i32,
    pub amount: i32,
    pub total: f32,
}

impl OrderItem {
    pub fn calculate_total(ingredient_id: u32, amount: u32) -> QueryResult<f32> {
        let ingredient_id = ingredient_id as i32;
        let amount = amount as i32;

        let conn = database::establish_connection();

        let ingredient: database::Ingredient =
            ingredients::table.find(ingredient_id).first(&conn)?;

        Ok(ingredient.price * amount as f32)
    }
}
