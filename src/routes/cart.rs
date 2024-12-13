use actix_web::web;

use crate::controllers::cart::{add_to_cart, remove_from_cart,clear_cart,get_cart,calculate_cart_total};


pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cart/{user_id}")
            .route("/add", web::post().to(add_to_cart))
            .route("/remove/{product_id}", web::delete().to(remove_from_cart))
            .route("/clear", web::delete().to(clear_cart))
            .route("", web::get().to(get_cart))
            .route("/total", web::get().to(calculate_cart_total)),
    );
}
