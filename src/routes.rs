use crate::vars;

/**
 * The root route.
 */
#[get("/")]
pub fn root() -> String {
    return object!{
        version: vars::API_VERSION
    }.dump();
}

/**
 * Gets a user by ID.
 */
#[get("/users/<id>")]
pub fn get_user(id: String) -> String {
    return object!{
        id: id
    }.dump();
}
