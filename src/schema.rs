// @generated automatically by Diesel CLI.

pub mod sql_types {
    use serde::{Deserialize, Serialize};

    #[derive(
        Debug,
        diesel::query_builder::QueryId,
        Clone,
        diesel::sql_types::SqlType,
        Serialize,
        Deserialize,
        PartialEq,
    )]
    #[diesel(postgres_type(name = "roles"))]
    #[serde(untagged)]
    pub enum Roles {
        Admin,
        Doctor,
        Patient,
    }

    impl Roles {
        fn as_str(&self) -> &'static str {
            match self {
                Roles::Admin => "admin",
                Roles::Doctor => "doctor",
                Roles::Patient => "patient",
            }
        }
    }
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
