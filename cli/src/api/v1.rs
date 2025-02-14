use std::sync::Arc;

use crate::state::ApplicationState;

use super::handlers;
use crate::api::middleware::jwt::auth;
use axum::routing::{get, post};
use axum::{middleware, Router};

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/hello",
            get(handlers::hello::hello).with_state(state.clone()),
        )
        .route(
            "/login",
            post(handlers::login::login).with_state(state.clone()),
        )
        .route(
            "/dogs",
            post(handlers::dogs::create)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(state, auth)),
        )
}
