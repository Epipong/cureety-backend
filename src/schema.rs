// @generated automatically by Diesel CLI.

pub mod sql_types {
    use diesel::sql_types::SqlType;

    #[derive(SqlType, Debug)]
    #[diesel(postgres_type(name = "roles"))]
    pub struct Roles;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Roles;

    users (id) {
        id -> Uuid,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 122]
        hash -> Varchar,
        role -> Roles,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
