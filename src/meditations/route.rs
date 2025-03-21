use crate::meditations::handler::*;
use actix_web::web;

pub fn meditations_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/meditations")
            .route("", web::post().to(create_meditation))
            .route("", web::get().to(get_meditations))
            .route("/{id}", web::get().to(get_meditation))
            .route("/{id}", web::put().to(update_meditation))
            .route("/{id}", web::delete().to(delete_meditation)),
    );
}
