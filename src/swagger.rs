use utoipa::OpenApi;
use utoipauto::utoipauto;

#[utoipauto(paths = "./src/web")]
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "IDK Backend")
    )
)]
pub struct ApiDoc;
