table! {
    use diesel::sql_types::*;
    use crate::models::user::*;

    task_status_for_user (id) {
        id -> Uuid,
        user_id -> Uuid,
        status_name -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::user::*;

    task_types_for_user (id) {
        id -> Uuid,
        user_id -> Uuid,
        type_name -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::user::*;

    tasks (id) {
        id -> Uuid,
        user_id -> Uuid,
        parent_id -> Nullable<Uuid>,
        title -> Varchar,
        task_description -> Text,
        created_at -> Timestamptz,
        status_name -> Nullable<Uuid>,
        type_name -> Nullable<Uuid>,
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

joinable!(task_status_for_user -> users (user_id));
joinable!(task_types_for_user -> users (user_id));
joinable!(tasks -> task_status_for_user (status_name));
joinable!(tasks -> task_types_for_user (type_name));
joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    task_status_for_user,
    task_types_for_user,
    tasks,
    users,
);
