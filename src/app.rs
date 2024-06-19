use std::sync::Arc;

use axum::Router;

use crate::{state::AppState, swagger::ApiDoc, web::users::routes::users_routes};
use utoipa::OpenApi;

pub async fn app(state: Arc<AppState>) -> Router {
    let swagger = utoipa_swagger_ui::SwaggerUi::new("/swagger")
        .url("/api-doc/openapi.json", ApiDoc::openapi());

    Router::new().merge(swagger).nest_service(
        "/api/v1",
        Router::new()
            .nest("/users", users_routes())
            .with_state(state),
    )
}
