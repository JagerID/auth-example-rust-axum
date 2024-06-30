use std::sync::Arc;

use axum::{middleware, Router};
use utoipa_swagger_ui::{Config, Url};

use crate::{
    config::cors::init_cors_layer,
    state::AppState,
    swagger::{ApiDocAuth, ApiDocProjects, ApiDocStats, ApiDocUsers},
    web::{
        auth::routes::auth_routes,
        error::handle_404,
        guard::{auth_guard, blocked_guard},
        profile::routes::profile_routes,
        projects::routes::projects_routes,
        stats::routes::stats_routes,
        users::routes::users_routes,
    },
};
use utoipa::OpenApi;

pub async fn app(state: Arc<AppState>) -> Router {
    let swagger = utoipa_swagger_ui::SwaggerUi::new("/swagger")
        .urls(vec![
            (
                Url::with_primary("Auth", "/api-docs/openapi-auth.json", true),
                ApiDocAuth::openapi(),
            ),
            (
                Url::new("Users", "/api-docs/openapi-users.json"),
                ApiDocUsers::openapi(),
            ),
            (
                Url::new("Projects", "/api-docs/openapi-projects.json"),
                ApiDocProjects::openapi(),
            ),
            (
                Url::new("Stats", "/api-docs/openapi-stats.json"),
                ApiDocStats::openapi(),
            ),
        ])
        .config(
            Config::default()
                .display_request_duration(true)
                .persist_authorization(true),
        );

    Router::new().merge(swagger).nest_service(
        "/api",
        Router::new()
            .nest("/users", users_routes(&state))
            .nest("/profile", profile_routes())
            .nest("/projects", projects_routes())
            .nest("/stats", stats_routes())
            .route_layer(middleware::from_fn_with_state(state.clone(), auth_guard))
            .route_layer(middleware::from_fn_with_state(state.clone(), blocked_guard))
            .nest("/auth", auth_routes())
            .with_state(state)
            .layer(init_cors_layer())
            .fallback(handle_404),
    )
}
