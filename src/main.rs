#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate json;

pub mod routes;
pub mod vars;

/**
 * The main function.
 */
fn main() {
    // Mount routes & run web server
    rocket::ignite().mount("/", routes![
        routes::root,
        routes::get_user
    ]).launch();
}
