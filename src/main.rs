#[macro_use]
extern crate rocket;

use burgershop::moderator;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![])
        .mount("/mod", routes![moderator::fill_ingredients])
}
