use std::sync::Arc;

use axum::{middleware, Router};

use crate::{
    state::AppState,
    swagger::ApiDoc,
    web::{guard::auth_guard, users::routes::users_routes},
};
use utoipa::OpenApi;

pub async fn app(state: Arc<AppState>) -> Router {
    let swagger = utoipa_swagger_ui::SwaggerUi::new("/swagger")
        .url("/api-doc/openapi.json", ApiDoc::openapi());

    Router::new().merge(swagger).nest_service(
        "/api/v1",
        Router::new()
            .nest("/users", users_routes())
            .route_layer(middleware::from_fn(auth_guard))
            .with_state(state),
    )
}
