use crate::{
    config::Connection,
    errors::{ErrorKind, ServiceError},
    schema::users::{self, dsl::*},
};
use actix_web::{error::ErrorUnauthorized, Error, FromRequest};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::future::Future;
use std::pin::Pin;

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

#[derive(Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub id: uuid::Uuid,
    pub username: String,
    pub user_role: UserRole,
}

impl From<Claims> for UserInfo {
    fn from(u: Claims) -> Self {
        Self {
            username: u.name,
            id: uuid::Uuid::parse_str(&u.sub).unwrap(),
            user_role: u.role,
        }
    }
}

impl From<User> for UserInfo {
    fn from(u: User) -> Self {
        Self {
            username: u.username,
            id: u.id,
            user_role: u.user_role,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthorisedUser(pub UserInfo);

impl FromRequest for AuthorisedUser {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<AuthorisedUser, Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let jwt = req.headers().get("Authorization").cloned();
        if let Some(jwt) = jwt {
            Box::pin(async move {
                let jwt: Vec<_> = jwt.to_str().unwrap().split_ascii_whitespace().collect();
                jsonwebtoken::decode::<Claims>(
                    jwt[1],
                    &DecodingKey::from_secret(b"secret"),
                    &Validation::new(Algorithm::HS512),
                )
                .map(|cl| cl.claims.into())
                .map(|u| AuthorisedUser(u))
                .map_err(|err| {println!("{:?}",err); ErrorUnauthorized("Invalid token")})
            })
        } else {
            Box::pin(async { Err(ErrorUnauthorized("Invalid auth type")) })
        }
    }
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
