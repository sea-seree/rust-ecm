use crate::error::ApiError;

#[derive(Clone)]
pub struct AppConfig {
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, ApiError> {
        // เพิ่ม logging เพื่อติดตาม
        println!("กำลังโหลด AppConfig...");
        
        std::env::var("JWT_SECRET")
            .map(|secret| {
                println!("โหลด JWT_SECRET สำเร็จ");
                Self { jwt_secret: secret }
            })
            .map_err(|_| {
                println!("โหลด JWT_SECRET ล้มเหลว");
                ApiError::AuthenticationError(
                    "ไม่พบ JWT_SECRET ในตัวแปรสภาพแวดล้อม".to_string()
                )
            })
    }
}