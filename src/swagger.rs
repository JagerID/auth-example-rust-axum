use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipauto::utoipauto;

#[utoipauto(paths = "./src/web/users, ./src/web/profile")]
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "IDK Backend (users part)")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDocUsers;

#[utoipauto(paths = "./src/web/auth")]
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "IDK Backend (auth part)")
    ),
    components(
        schemas(
            crate::web::users::dto::FilteredUser,
            crate::web::users::dto::CreateUserDto
        )
    )
    modifiers(&SecurityAddon)
)]
pub struct ApiDocAuth;

#[utoipauto(paths = "./src/web/stats")]
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "IDK Backend (stats part)")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDocStats;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
