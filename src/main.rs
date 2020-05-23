#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate json;

/**
 * The current API version.
 */
const API_VERSION: &str = "1.0.0";

/**
 * The root rout
 */
#[get("/")]
fn root() -> String {
    return object!{
        version: API_VERSION
    }.dump();
}

/**
 * Gets a user by ID.
 */
#[get("/users/<id>")]
fn get_user(id: String) -> String {
    return object!{
        id: id
    }.dump();
}

/**
 * The main function.
 */
fn main() {
    // Mount routes & run web server
    rocket::ignite().mount("/", routes![
        root,
        get_user
    ]).launch();
}
