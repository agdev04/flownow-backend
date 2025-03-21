use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

use crate::db::establish_connection;
use crate::meditations::model::{Meditation, NewMeditation, UpdateMeditation};
use crate::schema::meditations;

pub async fn create_meditation(new_meditation: web::Json<NewMeditation>) -> Result<HttpResponse> {
    let mut conn = establish_connection();

    let result = diesel::insert_into(meditations::table)
        .values(&*new_meditation)
        .get_result::<Meditation>(&mut conn);

    match result {
        Ok(meditation) => Ok(HttpResponse::Created().json(json!({
            "status": "success",
            "data": meditation
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Error creating meditation"
        }))),
    }
}

pub async fn get_meditations() -> Result<HttpResponse> {
    let mut conn = establish_connection();

    let result = meditations::table
        .select(Meditation::as_select())
        .load(&mut conn);

    match result {
        Ok(meditations) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "data": meditations
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Error fetching meditations"
        }))),
    }
}

pub async fn get_meditation(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = establish_connection();

    let result = meditations::table
        .find(*id)
        .select(Meditation::as_select())
        .first(&mut conn);

    match result {
        Ok(meditation) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "data": meditation
        }))),
        Err(_) => Ok(HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Meditation not found"
        }))),
    }
}

pub async fn update_meditation(
    id: web::Path<i32>,
    meditation: web::Json<UpdateMeditation>,
) -> Result<HttpResponse> {
    let mut conn = establish_connection();

    let result = diesel::update(meditations::table.find(*id))
        .set(&*meditation)
        .get_result::<Meditation>(&mut conn);

    match result {
        Ok(updated_meditation) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "data": updated_meditation
        }))),
        Err(_) => Ok(HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Meditation not found"
        }))),
    }
}

pub async fn delete_meditation(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = establish_connection();

    let result = diesel::delete(meditations::table.find(*id)).execute(&mut conn);

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Meditation deleted successfully"
        }))),
        Err(_) => Ok(HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Meditation not found"
        }))),
    }
}
