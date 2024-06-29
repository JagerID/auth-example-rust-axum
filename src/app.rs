use std::sync::Arc;

use axum::{middleware, Router};
use utoipa_swagger_ui::Config;

use crate::{
    config::cors::init_cors_layer,
    state::AppState,
    swagger::ApiDoc,
    web::{
        auth::routes::auth_routes, error::handle_404, guard::{auth_guard, blocked_guard}, profile::routes::profile_routes, stats::routes::stats_routes, users::routes::users_routes
    },
};
use utoipa::OpenApi;

pub async fn app(state: Arc<AppState>) -> Router {
    let swagger = utoipa_swagger_ui::SwaggerUi::new("/swagger")
        .url("/api-doc/openapi.json", ApiDoc::openapi())
        .config(
            Config::default()
                .filter(true)
                .display_request_duration(true)
                .persist_authorization(true),
        );

    Router::new().merge(swagger).nest_service(
        "/api",
        Router::new()
            .nest("/users", users_routes(&state))
            .nest("/profile", profile_routes())
            .nest("/stats", stats_routes())
            .route_layer(middleware::from_fn_with_state(state.clone(), auth_guard))
            .route_layer(middleware::from_fn_with_state(state.clone(), blocked_guard))
            .nest("/auth", auth_routes())
            .with_state(state)
            .layer(init_cors_layer())
            .fallback(handle_404),
    )
}
