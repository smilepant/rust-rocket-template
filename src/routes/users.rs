use rocket::{get, post, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec};
use crate::models::user::User;
use rocket_okapi::settings::OpenApiSettings;
#[openapi(tag = "Users")]
#[get("/")]
pub fn get_all_users() -> Json<Vec<User>> {
    Json(vec![User {
        user_id: 42,
        username: "bob".to_owned(),
        email: None,
    }])
}

#[openapi(tag = "Users")]
#[get("/<id>")]
pub fn get_user(id: u64) -> Option<Json<User>> {
    Some(Json(User {
        user_id: id,
        username: "bob".to_owned(),
        email: None,
    }))
}

#[openapi(tag = "Users")]
#[get("/example?<user_id>&<name>&<email>")]
pub fn get_user_by_name(user_id: u64, name: String, email: Option<String>) -> Option<Json<User>> {
    Some(Json(User {
        user_id,
        username: name,
        email,
    }))
}

#[openapi(tag = "Users")]
#[post("/", data = "<user>")]
pub fn create_user(user: Json<User>) -> Json<User> {
    user
}

#[openapi(skip)]
#[get("/hidden")]
pub fn hidden() -> Json<&'static str> {
    Json("Hidden from swagger!")
}

pub fn get_user_routes(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_all_users, get_user, get_user_by_name, create_user]
}