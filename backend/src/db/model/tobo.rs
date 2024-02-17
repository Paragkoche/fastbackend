use rocket::serde::{Deserialize, Serialize};

use diesel::prelude::*;

use crate::db::schema::todos;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = todos)]
pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
    pub due_date: Option<String>,
    pub completed: bool,
}
