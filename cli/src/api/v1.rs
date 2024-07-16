use std::sync::Arc;

use crate::state::ApplicationState;

use super::handlers;
use axum::routing::{get, post};
use axum::Router;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/hello",
            get(handlers::hello::hello).with_state(state.clone()),
        )
        .route("/login", post(handlers::login::login).with_state(state))
}
