use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:zxl12345.@localhost:3306/leite".to_string());

    let db = Database::connect(database_url).await?;
    Ok(db)
}

pub async fn create_user(db: &DatabaseConnection, name: String, email: String) -> Result<Model, DbErr> {
    let now = chrono::Utc::now();
    let user = ActiveModel {
        name: Set(name),
        email: Set(email),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let res = user.insert(db).await?;
    Ok(res)
}

pub async fn update_user(db: &DatabaseConnection, id: i32, name: Option<String>, email: Option<String>) -> Result<Model, DbErr> {
    let mut user: ActiveModel = Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("User not found".to_string()))
        .map(Into::into)?;

    if let Some(name) = name {
        user.name = Set(name);
    }
    if let Some(email) = email {
        user.email = Set(email);
    }
    user.updated_at = Set(chrono::Utc::now());

    let res = user.update(db).await?;
    Ok(res)
}

pub async fn delete_user(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
    let res = Entity::delete_by_id(id).exec(db).await?;
    Ok(res)
}

pub async fn find_user_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, DbErr> {
    let user = Entity::find_by_id(id).one(db).await?;
    Ok(user)
}