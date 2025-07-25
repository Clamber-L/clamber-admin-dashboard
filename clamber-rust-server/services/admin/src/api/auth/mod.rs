use crate::api::auth::service::*;
use crate::app_state::AppState;
use axum::routing::{get, post};
use axum::Router;

mod constants;
mod entity;
mod service;

pub fn sys_auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/user", get(user_info))
        .route("/permission_menu", get(permission_menu))
        .route("/save_permission", post(save_permission))
}
