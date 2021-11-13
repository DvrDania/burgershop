#[get("/ingredients")]
pub fn get_ingredients() {
    burgershop::Ingredient::get();
}
