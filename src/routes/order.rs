use actix_web::web;

use crate::controllers::order::{create_order, get_order_details, get_order_history, update_order_status};




pub fn configure_order_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .route("/{user_id}/create", web::post().to(create_order))
            .route("/{order_id}/details", web::get().to(get_order_details))
            .route("/{user_id}/history", web::get().to(get_order_history))
            .route("/{order_id}/status", web::put().to(update_order_status)),
    );
}
