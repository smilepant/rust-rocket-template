use rocket::{http::Status, outcome::Outcome, request::{self, FromRequest, Request}};
use rocket_okapi::{okapi::openapi3::{SecurityRequirement, SecuritySchemeData}, rapidoc::ApiKeyLocation, request::{OpenApiFromRequest, RequestHeaderInput}};
use rocket_okapi::okapi::openapi3::{SecurityScheme};

pub struct Authenticated;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authenticated {
    type Error = std::io::Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(token) = request.headers().get_one("Authorization") {
            if token == "Bearer valid_token" {
                Outcome::Success(Authenticated)
            } else {
                Outcome::Error((Status::Unauthorized, std::io::Error::new(std::io::ErrorKind::Other, "Unauthorized")))
            }
        } else {
            Outcome::Error((Status::Unauthorized, std::io::Error::new(std::io::ErrorKind::Other, "Unauthorized")))
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for Authenticated {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let security_scheme: SecurityScheme = SecurityScheme {
            data: SecuritySchemeData::ApiKey {
                name: "Authorization".to_owned(),
                location: ApiKeyLocation::Header.to_string(),
            },
            description: Some("Requires a valid authorization token".to_owned()),
            extensions: Default::default(),
        };
        
        let mut security_requirement = SecurityRequirement::new();
        security_requirement.insert("BearerAuth".to_owned(), Vec::new());
        
        Ok(RequestHeaderInput::Security(
            "BearerAuth".to_owned(),
            security_scheme,
            security_requirement,
        ))
    }
}
