use crate::schema::users::{self, dsl::*};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

#[derive(Clone, Debug, DbEnum, Serialize, Deserialize)]
#[DieselType = "User_role_t"]
pub enum UserType {
    Admin,
    User,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub user_role: UserType,
    pub secret: String,
}
