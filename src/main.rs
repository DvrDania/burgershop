#[macro_use]
extern crate rocket;

pub mod api_response;
pub mod customer;
pub mod moderator;

use customer::*;
use moderator::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        // routes that are public and do not need authentication
        .mount("/", routes![get_burgers])
        // routes that are private and need authentication
        .mount(
            "/",
            routes![
                set_ingredients,
                update_ingredient,
                delete_ingredient,
                set_tables
            ],
        )
}
