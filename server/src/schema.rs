table! {
    use diesel::sql_types::*;
    use crate::models::user::*;

    tasks (id) {
        id -> Uuid,
        owner_id -> Uuid,
        parent_id -> Nullable<Uuid>,
        title -> Varchar,
        task_description -> Text,
    }
}

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

joinable!(tasks -> users (owner_id));

allow_tables_to_appear_in_same_query!(tasks, users,);
