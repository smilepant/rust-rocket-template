use crate::models::response::ErrorResponse;
use rocket::http::Status;
use rocket::serde::json::Json as RocketJson;
use rocket::{catch, catchers, Request};

#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> RocketJson<ErrorResponse> {
    RocketJson(ErrorResponse {
        success: false,
        message: format!("{}", Status::new(status.code)),
        code: status.code,
    })
}

pub fn get_catchers() -> Vec<rocket::Catcher> {
    catchers![
        default_catcher
    ]
}
