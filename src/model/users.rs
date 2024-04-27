use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
};

use serde::{Deserialize, Serialize};
// use uuid::Uuid;

#[derive(Debug, AsExpression, FromSqlRow, Serialize)]
#[diesel(sql_type = crate::schema::sql_types::Language)]
pub enum Language {
    En,
    Jp,
}

impl ToSql<crate::schema::sql_types::Language, Pg> for Language {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Language::En => out.write_all(b"EN")?,
            Language::Jp => out.write_all(b"JP")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Language, Pg> for Language {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"EN" => Ok(Language::En),
            b"JP" => Ok(Language::Jp),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub language: Language,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub id: String,
    pub username: String,
}
