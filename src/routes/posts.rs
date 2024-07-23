use crate::models::post::Post;
use rocket::get;
use rocket::serde::json::Json;
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings
};

#[openapi(tag = "Posts")]
#[get("/post_by_query?<post..>")]
pub fn create_post_by_query(post: Post) -> Option<Json<Post>> {
    Some(Json(post))
}
pub fn get_post_routes(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: create_post_by_query]
}
