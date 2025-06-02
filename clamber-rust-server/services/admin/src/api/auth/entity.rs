use lib_entity::mysql::sys_user::Model;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default, Clone)]
pub struct LoginParam {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub phone: String,
    pub access_token: String,
}

impl LoginResponse {
    pub(crate) fn new(user: Model, access_token: String) -> Self {
        Self {
            id: user.id,
            username: user.username,
            avatar: user.avatar,
            email: user.email,
            phone: user.phone,
            access_token,
        }
    }
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PermissionMenuResponse {
    pub id: String,
    pub path: String,
    pub name: String,
    pub component: String,
    pub meta: PermissionMenuMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<PermissionMenuResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PermissionMenuMeta {
    pub title: String,
    pub icon: Option<String>,
    pub keep_alive: bool,
    pub auth_list: Option<Vec<String>>,
}
