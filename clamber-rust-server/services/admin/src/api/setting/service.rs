use crate::api::setting::constants::SettingType;
use crate::api::setting::entity::{EditSettingParam, LoginImageResponse};
use crate::app_state::AppState;
use axum::extract::State;
use lib_core::jwt::JwtUser;
use lib_core::{generate_snowflake_id, ApiResult, ExtractJson};
use lib_entity::mysql::prelude::SysSetting;
use lib_entity::mysql::sys_setting;
use lib_entity::mysql::sys_setting::ActiveModel;
use lib_utils::result::{ok_result, ok_result_with_none};
use sea_orm::prelude::Expr;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, Set};

pub async fn login_image(State(state): State<AppState>) -> ApiResult<LoginImageResponse> {
    let result = SysSetting::find()
        .filter(Expr::col(sys_setting::Column::Type).eq(1))
        .one(&state.mysql_client)
        .await?;
    if let Some(res) = result {
        Ok(ok_result(LoginImageResponse::new(res.value)))
    } else {
        Ok(ok_result(LoginImageResponse::none()))
    }
}

pub async fn edit_login_image(
    State(state): State<AppState>,
    _user: JwtUser,
    ExtractJson(param): ExtractJson<EditSettingParam>,
) -> ApiResult<String> {
    let EditSettingParam { value } = param;
    println!("naive_local:{:?}", Local::now().naive_local());

    let setting_type = SettingType::LOGIN_IMAGE.value();
    let model = SysSetting::find()
        .filter(Expr::col(sys_setting::Column::Type).eq(setting_type))
        .one(&state.mysql_client)
        .await?;

    if let Some(setting) = model {
        // 修改
        let mut setting_active: ActiveModel = setting.into();
        setting_active.value = Set(value);
        setting_active.updated_time = Set(Local::now().naive_local());
        setting_active.update(&state.mysql_client).await?;
        Ok(ok_result_with_none())
    } else {
        let model = ActiveModel {
            id: Set(generate_snowflake_id()?),
            created_time: Set(Local::now().naive_local()),
            updated_time: Set(Local::now().naive_local()),
            r#type: Set(setting_type),
            value: Set(value),
        };
        model.insert(&state.mysql_client).await?;
        Ok(ok_result_with_none())
    }
}
