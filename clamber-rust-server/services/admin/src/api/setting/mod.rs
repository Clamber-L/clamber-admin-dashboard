mod constants;
mod entity;
mod service;

use crate::api::setting::service::{edit_login_image, login_image};
use crate::app_state::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn sys_setting_router() -> Router<AppState> {
    Router::new()
        .route("/login_image", get(login_image))
        .route("/edit_setting", post(edit_login_image))
}
