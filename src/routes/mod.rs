pub mod auth;
pub mod products;
pub mod cart;
pub mod order;

pub use auth::configure_auth_routes;
pub use products::configure_product_routes;
pub use cart::configure_cart_routes;
pub use order::configure_order_routes;
