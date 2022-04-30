table! {
    use diesel::sql_types::*;
    use crate::models::user::*;

    users (id) {
        id -> Uuid,
        username -> Varchar,
        user_role -> User_role_t,
        secret -> Text,
    }
}
