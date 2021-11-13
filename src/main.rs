#[macro_use]
extern crate rocket;

use burgershop::moderator::*;
use burgershop::public::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        // routes that are public and do not need authentication
        .mount("/", routes![get_ingredients])
        // routes that are private and need authentication
        .mount(
            "/",
            routes![set_ingredients, update_ingredient, delete_ingredient],
        )
}
