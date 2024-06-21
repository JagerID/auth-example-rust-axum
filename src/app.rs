use std::sync::Arc;

use axum::{middleware, Router};

use crate::{
    state::AppState,
    swagger::ApiDoc,
    web::{auth::routes::auth_routes, guard::auth_guard, users::routes::users_routes},
};
use utoipa::OpenApi;

pub async fn app(state: Arc<AppState>) -> Router {
    let swagger = utoipa_swagger_ui::SwaggerUi::new("/swagger")
        .url("/api-doc/openapi.json", ApiDoc::openapi());

    Router::new().merge(swagger).nest_service(
        "/api",
        Router::new()
            .nest("/users", users_routes())
            .route_layer(middleware::from_fn_with_state(state.clone(), auth_guard))
            .nest("/auth", auth_routes())
            .with_state(state),
    )
}
