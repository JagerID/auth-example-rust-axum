use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn init_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH])
        .allow_origin(Any)
}
