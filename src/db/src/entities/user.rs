use sea_orm::{entity::prelude::*, IntoActiveModel, QueryOrder};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub balance: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub fn create_user(id: &str) -> Model {
    Model {
        id: id.to_string(),
        balance: 0,
    }
}

pub async fn get_user(db: &DatabaseConnection, id: &str) -> Result<Model, DbErr> {
    if let Some(user) = Entity::find_by_id(id.to_string()).one(db).await? {
        return Ok(user);
    }

    let user = create_user(id).into_active_model();
    let user = user.insert(db).await?;

    Ok(user)
}

pub async fn find_highest_users(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    let mut paginator = Entity::find()
        .order_by_desc(Column::Balance)
        .paginate(db, 5);

    Ok(paginator.fetch().await?)
}
