use crate::schema::categories;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub name: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = categories)]
pub struct UpdateCategory {
    pub name: Option<String>,
}
