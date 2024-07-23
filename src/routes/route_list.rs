use rocket_okapi::openapi_get_routes;
use crate::routes::users;
use crate::routes::posts;

pub fn get_openapi_routes() -> Vec<rocket::Route> {
    openapi_get_routes![
        //users routes
        users::get_all_users,
        users::get_user,
        users::get_user_by_name,
        users::create_user,
        users::hidden,

        //post routes
        posts::create_post_by_query,
        // Add any other routes here
    ]
}
