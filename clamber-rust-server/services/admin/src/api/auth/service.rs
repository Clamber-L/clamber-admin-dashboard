use crate::api::auth::entity::{
    LoginParam, LoginResponse, PermissionMenuMeta, PermissionMenuResponse,
};
use crate::app_state::AppState;
use axum::extract::State;
use lib_core::jwt::{generate_jwt, JwtUser};
use lib_core::{ApiResult, ExtractJson};
use lib_entity::mysql::prelude::{SysPermission, SysRolePermission, SysUser, SysUserRole};
use lib_entity::mysql::{sys_permission, sys_role_permission, sys_user, sys_user_role};
use lib_utils::password::verify_password;
use lib_utils::result::{error_result, ok_result, ok_result_with_none};
use sea_orm::prelude::Expr;
use sea_orm::{EntityTrait, QueryFilter};
use std::collections::{HashMap, HashSet};

pub async fn login(
    State(state): State<AppState>,
    ExtractJson(param): ExtractJson<LoginParam>,
) -> ApiResult<LoginResponse> {
    if let Some(user) = SysUser::find()
        .filter(Expr::col(sys_user::Column::Username).eq(param.username))
        .one(&state.mysql_client)
        .await?
    {
        if verify_password(&param.password, &user.password) {
            let access_token = generate_jwt(JwtUser {
                id: user.id.clone(),
            });
            Ok(ok_result(LoginResponse::new(user, access_token)))
        } else {
            Ok(error_result("密码错误"))
        }
    } else {
        Ok(error_result("用户不存在"))
    }
}

pub async fn user_info(State(_state): State<AppState>, _user: JwtUser) -> ApiResult<String> {
    Ok(ok_result_with_none())
}

pub async fn permission_menu(
    State(state): State<AppState>,
    user: JwtUser,
) -> ApiResult<Vec<PermissionMenuResponse>> {
    // 获取用户角色
    let role_ids: Vec<String> = SysUserRole::find()
        .filter(Expr::col(sys_user_role::Column::UserId).eq(user.id))
        .all(&state.mysql_client)
        .await?
        .into_iter()
        .map(|r| r.role_id)
        .collect();

    // 角色对应权限
    let perm_ids: HashSet<String> = SysRolePermission::find()
        .filter(Expr::col(sys_role_permission::Column::RoleId).is_in(role_ids))
        .all(&state.mysql_client)
        .await?
        .into_iter()
        .map(|p| p.permission_id)
        .collect();

    // 权限详情
    let permissions = SysPermission::find()
        .filter(Expr::col(sys_permission::Column::Id).is_in(perm_ids))
        .all(&state.mysql_client)
        .await?;

    // 组装数据
    let menus: Vec<PermissionMenuResponse> = permissions
        .into_iter()
        .map(|perm| PermissionMenuResponse {
            id: perm.id,
            path: perm.path,
            name: perm.name,
            component: perm.component,
            meta: PermissionMenuMeta {
                title: perm.title,
                icon: perm.icon,
                keep_alive: perm.keep_alive,
                auth_list: None,
            },
            children: None,
            parent_id: perm.parent_id,
        })
        .collect();

    println!("menus:{:?}", menus);

    // 构建树结构
    let tree = build_menu_tree(menus);
    println!("tree:{:?}", tree);
    Ok(ok_result(tree))
}

fn build_menu_tree(menus_list: Vec<PermissionMenuResponse>) -> Vec<PermissionMenuResponse> {
    let mut map: HashMap<String, PermissionMenuResponse> = HashMap::new();

    let mut roots = vec![];

    for mut item in menus_list {
        item.children = Some(vec![]);
        map.insert(item.clone().id, item);
    }

    // 找到有parent_id的节点
    let keys: Vec<String> = map.keys().cloned().collect();
    for id in keys {
        if let Some(parent_id) = map.get(&id).and_then(|item| item.parent_id.clone()) {
            // 移除子节点，再将其加入到对应的父节点中
            if let Some(child) = map.remove(&id) {
                if let Some(parent) = map.get_mut(&parent_id) {
                    parent.children.get_or_insert(Vec::new()).push(child);
                }
            }
        }

        // 有parent_id的子节点
        // if let (Some(parent_id), Some(child)) = (
        //     map.get(&id).and_then(|item| item.parent_id.clone()),
        //     map.remove(&id),
        // ) {
        //     if let Some(mut parent) = map.get_mut(&parent_id) {
        //         parent.children.get_or_insert(vec![]).push(child);
        //     }
        // }
    }

    // 处理其他的节点(顶层节点)
    for (_, item) in map {
        roots.push(item);
    }

    roots.sort_by(|a, b| a.id.cmp(&b.id));
    roots
}
