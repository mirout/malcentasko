use crate::{
    config::Connection,
    errors::{ErrorKind, ServiceError},
    schema::users::{self, dsl::*},
};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use jsonwebtoken::{Algorithm, EncodingKey, Header};

#[derive(Clone, Debug, DbEnum, Serialize, Deserialize)]
#[DieselType = "User_role_t"]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub user_role: UserRole,
    pub secret: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserCredentials {
    pub username: String,
    pub secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
    name: String,
    role: UserRole,
}

type Token = String;

impl User {
    pub fn signup(user: UserCredentials, conn: &Connection) -> Result<String, ServiceError> {
        if Self::find_user_by_username(&user.username, conn).is_err() {
            let user =
                bcrypt::hash(user.secret, bcrypt::DEFAULT_COST).map(|hash| UserCredentials {
                    secret: hash,
                    ..user
                })?;
            diesel::insert_into(users).values(&user).execute(conn)?;
            Ok(String::from("User was successfully added"))
        } else {
            Err(ServiceError::forbidden(format!(
                "User '{}' already exists",
                user.username
            )))
        }
    }

    pub fn login(user: UserCredentials, conn: &Connection) -> Result<Token, ServiceError> {
        match Self::find_user_by_username(&user.username, conn) {
            Ok(user_verified) if bcrypt::verify(user.secret, &user_verified.secret)? => {
                Ok(Self::generate_token(&user_verified))
            }
            Ok(_) => Err(ServiceError::new(
                ErrorKind::Forbidden,
                "Wrong password".to_string(),
            )),
            Err(_) => Err(ServiceError::no_such_user(user.username)),
        }
    }

    pub fn find_user_by_username(un: &str, conn: &Connection) -> QueryResult<User> {
        users.filter(username.eq(un)).get_result::<User>(conn)
    }

    fn generate_token(user: &User) -> String {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(60))
            .expect("Valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.id.to_string(),
            exp: expiration as usize,
            name: user.username.clone(),
            role: user.user_role.clone(),
        };

        let header = Header::new(Algorithm::HS512);
        jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(b"secret")).unwrap()
    }
}
