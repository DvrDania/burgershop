#[macro_use]
extern crate rocket;

use burgershop::moderator;
use burgershop::public;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![public::get_ingredients])
        .mount("/", routes![moderator::set_ingredients])
}
