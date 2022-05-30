use actix_web::dev::Service;
use diesel::prelude::*;
use crate::{
    config::Connection,
    errors::ServiceError,
    schema::{task_types_for_user, task_status_for_user},
    models::user::User,
};

#[derive(Debug, Identifiable, Associations, Queryable, Serialize, Deserialize)]
#[belongs_to(User)]
#[table_name = "task_types_for_user"]
pub struct TypeTable {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub type_name: Option<String>
}

#[derive(Debug, Identifiable, Associations, Queryable, Serialize, Deserialize)]
#[belongs_to(User)]
#[table_name = "task_status_for_user"]
pub struct StatusTable {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub status_name: Option<String>
}

impl StatusTable {
    pub fn get_status_by_name(value: &str, conn: &Connection) -> Result<StatusTable, ServiceError> {
        Ok(task_status_for_user::dsl::task_status_for_user
            .filter(task_status_for_user::dsl::status_name.eq(value))
            .get_result::<StatusTable>(conn)?
        )
    }
    pub fn create_status_by_name(value: &str, user_id: uuid::Uuid, conn: &Connection) -> Result<StatusTable, ServiceError> {
        Ok(diesel::insert_into(task_status_for_user::dsl::task_status_for_user)
            .values((task_status_for_user::dsl::user_id.eq(user_id), task_status_for_user::dsl::status_name.eq(value)))
            .get_result::<StatusTable>(conn)?
        )
    }
}

impl TypeTable {
    pub fn get_type_by_name(value: &str, conn: &Connection) -> Result<TypeTable, ServiceError> {
        Ok(task_types_for_user::dsl::task_types_for_user
            .filter(task_types_for_user::dsl::type_name.eq(value))
            .get_result::<TypeTable>(conn)?
        )
    }
    pub fn create_type_by_name(value: &str, user_id: uuid::Uuid, conn: &Connection) -> Result<TypeTable, ServiceError> {
        Ok(diesel::insert_into(task_types_for_user::dsl::task_types_for_user)
            .values((task_types_for_user::dsl::user_id.eq(user_id), task_types_for_user::dsl::type_name.eq(value)))
            .get_result::<TypeTable>(conn)?
        )
    }
}