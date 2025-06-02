use anyhow::Result;
use lib_core::{generate_snowflake_id, mysql_pool};
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{ActiveModelTrait, Set};

#[tokio::main]
async fn main() -> Result<()> {
    let ids = "1,
101,
102,
103,
2,
201,
2010,
202,
2021,
2022,
204,
205,
3,
301,
302,
303,
304,
4,
401,
4011,
4012,
4013,
402,
40201,
40202,
4020201,
40203,
4020301,
4020302,
402030201,
411,
4111,
4112,
4113";

    let connection = mysql_pool("mysql://root:Lsw%400516@47.95.179.146:3306/fish").await?;
    let ids_list: Vec<String> = ids
        .replace("\n", "")
        .split(',')
        .map(|x| x.to_string())
        .collect();
    for id in ids_list {
        println!("{:?}", id);
        let model = lib_entity::mysql::sys_role_permission::ActiveModel {
            id: Set(generate_snowflake_id()?),
            created_time: Set(Local::now().naive_local()),
            updated_time: Set(Local::now().naive_local()),
            role_id: Set("r1".to_string()),
            permission_id: Set(id),
        };

        model.insert(&connection).await?;
    }

    Ok(())
}
