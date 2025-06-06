use std::num::ParseIntError;

use axum::extract::rejection::{FormRejection, PathRejection, QueryRejection};
use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use lib_utils::json_response;
use lib_utils::result::{error_result, HttpResult};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal server error")]
    InternalServerError,

    #[error("{0}")]
    ServiceError(&'static str),

    #[error("sea-orm sql error")]
    DbError(#[from] sea_orm::DbErr),

    // #[error("mongo error")]
    // MongoError(#[from] mongodb::error::Error),
    #[error("parse error")]
    ParseError(#[from] chrono::ParseError),

    #[error("axum error")]
    AxumError(#[from] axum::Error),

    #[error("json deserialize error")]
    JsonDeserializeError(#[from] JsonRejection),

    #[error("path deserialize error")]
    PathDeserializationError(#[from] PathRejection),

    #[error("query deserialize error")]
    QueryDeserializationError(#[from] QueryRejection),

    #[error("form deserialize error")]
    FormDeserializeError(#[from] FormRejection),

    #[error("parse int error")]
    ParseIntError(#[from] ParseIntError),

    #[error("sonyflake error")]
    SonyflakeError(#[from] sonyflake::Error),

    #[error("derive_builder error")]
    JwtUserBuilderError(#[from] derive_builder::UninitializedFieldError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, body): (StatusCode, HttpResult<()>) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, error_result("资源没有找到")),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, error_result("没有权限")),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                error_result("服务器异常，请稍后重试"),
            ),
            AppError::ServiceError(error_str) => (StatusCode::OK, error_result(error_str)),
            AppError::DbError(sql_error) => {
                error!("sql error:{:?}", sql_error);
                (StatusCode::OK, error_result("操作失败"))
            }
            // AppError::MongoError(mongo_error) => {
            //     error!("mongo error:{:?}", mongo_error);
            //     (StatusCode::OK, error_result("查询失败"))
            // }
            AppError::ParseError(parse_error) => {
                error!("时间转换失败:{:?}", parse_error);
                (StatusCode::OK, error_result("参数错误"))
            }
            AppError::AxumError(axum_error) => {
                error!("axum error:{:?}", axum_error);
                (StatusCode::OK, error_result("参数错误"))
            }
            AppError::QueryDeserializationError(query_error) => {
                error!("query param error:{:?}", query_error.body_text());
                (StatusCode::OK, error_result("参数错误"))
            }
            AppError::PathDeserializationError(path_error) => {
                error!("path error:{:?}", path_error.body_text());
                (StatusCode::OK, error_result("参数错误"))
            }
            AppError::JsonDeserializeError(json_error) => {
                error!("json error:{:?}", json_error.body_text());
                (StatusCode::OK, error_result("参数错误"))
            }
            AppError::FormDeserializeError(form_error) => {
                error!("form error:{:?}", form_error.body_text());
                (StatusCode::OK, error_result("参数错误"))
            }
            AppError::ParseIntError(parse_error) => {
                error!("parse error:{:?}", parse_error);
                (StatusCode::OK, error_result("系统内部错误"))
            }
            AppError::SonyflakeError(sonyflake_error) => {
                error!("sonyflake error:{:?}", sonyflake_error);
                (StatusCode::OK, error_result("系统内部错误"))
            }
            AppError::JwtUserBuilderError(builder_error) => {
                error!("jwt user error:{:?}", builder_error);
                (StatusCode::OK, error_result("系统内部错误"))
            }
        };
        json_response(status, &body)
    }
}
