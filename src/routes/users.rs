use rocket::{get, post, routes, serde::json::Json};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::{Deserialize, Serialize};
use crate::models::user::User;
use rocket_okapi::openapi_get_routes;
#[openapi(tag = "Users")]
#[get("/user")]
pub fn get_all_users() -> Json<Vec<User>> {
    Json(vec![User {
        user_id: 42,
        username: "bob".to_owned(),
        email: None,
    }])
}

#[openapi(tag = "Users")]
#[get("/user/<id>")]
pub fn get_user(id: u64) -> Option<Json<User>> {
    Some(Json(User {
        user_id: id,
        username: "bob".to_owned(),
        email: None,
    }))
}

#[openapi(tag = "Users")]
#[get("/user_example?<user_id>&<name>&<email>")]
pub fn get_user_by_name(user_id: u64, name: String, email: Option<String>) -> Option<Json<User>> {
    Some(Json(User {
        user_id,
        username: name,
        email,
    }))
}

#[openapi(tag = "Users")]
#[post("/user", data = "<user>")]
pub fn create_user(user: Json<User>) -> Json<User> {
    user
}

#[openapi(skip)]
#[get("/hidden")]
pub fn hidden() -> Json<&'static str> {
    Json("Hidden from swagger!")
}


