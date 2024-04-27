use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Debug, Clone)]
#[diesel(table_name = crate::schema::events)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub user_id: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::events)]
pub struct NewEvent {
    pub id: String,
    pub name: String,
    pub user_id: String,
}
