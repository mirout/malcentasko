use std::error::Error as StdError;
use std::fmt::Display;

use actix_web::HttpResponse;

#[derive(Debug)]
pub enum ErrorKind {
    Forbidden,
    DBError,
    InternalError,
    NotFound,
}

#[derive(Debug)]
pub struct ServiceError {
    kind: ErrorKind,
    msg: String,
    source: Option<Box<dyn std::error::Error + Send>>,
}

impl ServiceError {
    pub fn new(kind: ErrorKind, msg: String) -> Self {
        Self {
            kind,
            msg,
            source: None,
        }
    }
    pub fn no_such_user(username: String) -> Self {
        Self {
            kind: ErrorKind::NotFound,
            msg: format!("Can't find user: {}", username),
            source: None,
        }
    }

    pub fn forbidden(msg: String) -> Self {
        Self {
            kind: ErrorKind::Forbidden,
            msg,
            source: None,
        }
    }
}

impl StdError for ServiceError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<bcrypt::BcryptError> for ServiceError {
    fn from(err: bcrypt::BcryptError) -> Self {
        Self {
            kind: ErrorKind::InternalError,
            msg: String::from("error with Bcrypt"),
            source: Some(Box::new(err)),
        }
    }
}

impl From<diesel::result::Error> for ServiceError {
    fn from(err: diesel::result::Error) -> Self {
        Self {
            kind: ErrorKind::DBError,
            msg: String::from("error with database"),
            source: Some(Box::new(err)),
        }
    }
}

impl Into<HttpResponse> for ServiceError {
    fn into(self) -> HttpResponse {
        match self.kind {
            ErrorKind::Forbidden => HttpResponse::Forbidden(),
            ErrorKind::InternalError | ErrorKind::DBError => HttpResponse::InternalServerError(),
            ErrorKind::NotFound => HttpResponse::NotFound(),
        }
        .body(self.msg)
    }
}
