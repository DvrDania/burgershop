use rocket::serde::json::Json;

#[post("/ingredients", data = "<items>", format = "json")]
pub fn set_ingredients(items: Json<Vec<burgershop::Ingredient>>) {
    let items = items.into_inner();
    burgershop::Ingredient::set(items);
}

#[put("/ingredients/<id>", data = "<new_value>", format = "json")]
pub fn update_ingredient(id: u32, new_value: Json<burgershop::Ingredient>) {
    let new_value = new_value.into_inner();
    burgershop::Ingredient::update(id, new_value);
}

#[delete("/ingredients/<id>")]
pub fn delete_ingredient(id: u32) {
    burgershop::Ingredient::delete(id);
}
