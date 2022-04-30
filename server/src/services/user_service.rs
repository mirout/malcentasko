use crate::config::Connection;
use crate::errors::ServiceError;
use crate::models::user::{User, UserCredentials};

pub fn signup(user: UserCredentials, conn: &Connection) -> Result<String, ServiceError> {
    User::signup(user, conn)
}

pub fn login(user: UserCredentials, conn: &Connection) -> Result<String, ServiceError> {
    User::login(user, conn)
}
