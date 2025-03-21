use crate::schema::meditations;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = meditations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Meditation {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub category_id: i32,
    pub tags: String,
    pub script: String,
    pub image_url: String,
    pub audio_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = meditations)]
pub struct NewMeditation {
    pub title: String,
    pub description: String,
    pub category_id: i32,
    pub tags: String,
    pub script: String,
    pub image_url: String,
    pub audio_url: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = meditations)]
pub struct UpdateMeditation {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub tags: Option<String>,
    pub script: Option<String>,
    pub image_url: Option<String>,
    pub audio_url: Option<String>,
}
