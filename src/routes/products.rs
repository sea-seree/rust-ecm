use crate::controllers::product::{
    create_product, delete_product, get_product, get_products, update_product,
    update_product_status,
};
use actix_web::web;

pub fn configure_product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::get().to(get_products))
            .route("/{id}", web::get().to(get_product))
            .route("", web::post().to(create_product))
            .route("/{id}", web::put().to(update_product))
            .route("/{id}", web::delete().to(delete_product))
            .route("/{id}/status", web::put().to(update_product_status)),
    );
}
