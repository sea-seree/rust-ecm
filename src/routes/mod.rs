pub mod auth;
pub mod products;
pub mod cart;

pub use auth::configure_auth_routes;
pub use products::configure_product_routes;
pub use cart::configure_cart_routes;
