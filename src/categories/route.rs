use crate::categories::handler::{
    create_category, delete_category, get_categories, get_category, update_category,
};
use actix_web::web;

pub fn cagtegories_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/categories")
            .route("", web::post().to(create_category))
            .route("", web::get().to(get_categories))
            .route("/{id}", web::get().to(get_category))
            .route("/{id}", web::put().to(update_category))
            .route("/{id}", web::delete().to(delete_category)),
    );
}
