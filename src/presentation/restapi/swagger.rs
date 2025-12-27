use utoipa::Modify;
use utoipa::OpenApi;
use utoipa::openapi::security::HttpAuthScheme;
use utoipa::openapi::security::HttpBuilder;
use utoipa::openapi::security::SecurityScheme;

use crate::presentation::restapi::auth;
use crate::presentation::restapi::todo;
use crate::presentation::restapi::user;

struct JsonWebToken;

impl Modify for JsonWebToken {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

#[derive(Debug, OpenApi)]
#[openapi(
    servers((url = "/api/v1", description = "Base API v1")), 
    paths(
        auth::controller::register,
        auth::controller::login_with_email,
        auth::controller::refresh_access_token,
        auth::controller::whoami,
        auth::controller::logout,

        user::controller::delete_user,
        user::controller::find_all_user,
        user::controller::find_user_by_id,

        todo::controller::create_todo,
        todo::controller::update_todo,
        todo::controller::delete_todo,
        todo::controller::find_all_todo,
        todo::controller::find_todo_by_id,
        todo::controller::toggle_todo,
    ),
    modifiers(&JsonWebToken)
)]
pub struct ApiDoc;
