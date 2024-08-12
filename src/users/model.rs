// Generated by diesel_ext
use chrono::NaiveDateTime;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::{prelude::*, r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};
use std::io::Write;
use uuid::Uuid;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Deserialize, Serialize, Eq)]
#[diesel(sql_type = crate::schema::sql_types::Roles)]
pub enum Roles {
    Admin,
    Doctor,
    Patient,
}

impl ToSql<crate::schema::sql_types::Roles, Pg> for Roles {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Roles::Admin => out.write_all(b"admin")?,
            Roles::Doctor => out.write_all(b"doctor")?,
            Roles::Patient => out.write_all(b"patient")?,
        };
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Roles, Pg> for Roles {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"admin" => Ok(Roles::Admin),
            b"doctor" => Ok(Roles::Doctor),
            b"patient" => Ok(Roles::Patient),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl Roles {
    pub fn as_str(&self) -> &str {
        match self {
            Roles::Admin => "admin",
            Roles::Doctor => "doctor",
            Roles::Patient => "patient",
        }
    }
}

use crate::schema::users;
#[derive(
    Queryable, Selectable, Identifiable, Debug, PartialEq, Deserialize, Serialize, Insertable,
)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub hash: String,
    pub role: Roles,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub hash: String,
    pub role: Option<Roles>,
}