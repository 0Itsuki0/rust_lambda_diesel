// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "language"))]
    pub struct Language;
}

diesel::table! {
    events (id) {
        id -> Varchar,
        name -> Varchar,
        user_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Language;

    users (id) {
        id -> Varchar,
        username -> Varchar,
        language -> Language,
    }
}

diesel::joinable!(events -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    users,
);
