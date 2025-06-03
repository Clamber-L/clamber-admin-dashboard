use crate::api::auth::service::{login, permission_menu, user_info};
use crate::app_state::AppState;
use axum::routing::{get, post};
use axum::Router;

mod entity;
mod service;
mod constants;

pub fn sys_auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/user", get(user_info))
        .route("/permission_menu", get(permission_menu))
}
