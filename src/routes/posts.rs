use crate::models::post::Post;
use rocket::form::FromForm;
use rocket::{get, routes};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use rocket_okapi::{okapi::schemars::JsonSchema};
use rocket_okapi::openapi_get_routes;
use serde::{Deserialize, Serialize};
#[openapi(tag = "Posts")]
#[get("/post_by_query?<post..>")]
pub fn create_post_by_query(post: Post) -> Option<Json<Post>> {
    Some(Json(post))
}


