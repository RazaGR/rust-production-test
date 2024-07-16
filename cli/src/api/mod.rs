use std::sync::Arc;

use axum::Router;

use crate::state::ApplicationState;

mod handlers;
mod middleware;
mod request;
mod response;
mod v1;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new().nest("/v1", v1::configure(state))
}
