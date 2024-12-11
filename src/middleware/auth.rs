use crate::services::auth::Claims;
use actix_web::{
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use std::sync::Arc;
use std::task::{Context, Poll};

#[derive(Debug, Deserialize)]

// Middleware Struct
pub struct AuthMiddleware;

// Inner Struct ที่ทำหน้าที่เป็นตัว Middleware จริง
pub struct AuthMiddlewareMiddleware<S> {
    service: Arc<S>, // ใช้ Arc เพื่อแชร์ Service
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static + actix_web::body::MessageBody,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AuthMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware {
            service: Arc::new(service),
        })
    }
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static + actix_web::body::MessageBody,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            if req.path().starts_with("/auth") {
                return service.call(req).await.map(|res| res.map_into_boxed_body());
            }
            // ตรวจสอบ Header Authorization
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];
                        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");

                        // ตรวจสอบ JWT Token
                        match decode::<Claims>(
                            token,
                            &DecodingKey::from_secret(secret.as_bytes()),
                            &Validation::default(),
                        ) {
                            Ok(decoded) => {
                                // เพิ่ม User ID ลงใน Extensions
                                req.extensions_mut()
                                    .insert(decoded.claims.get_sub().to_string());

                                // ส่งต่อ Request ไปยัง Service
                                let response = service.call(req).await;
                                return response.map(|res| res.map_into_boxed_body());
                            }
                            Err(_) => {
                                return Ok(req.into_response(
                                    HttpResponse::Unauthorized()
                                        .body("Invalid or expired token")
                                        .map_into_boxed_body(),
                                ));
                            }
                        }
                    }
                }
            }

            // กรณีไม่มี Token
            Ok(req.into_response(
                HttpResponse::Unauthorized()
                    .body("Missing or invalid token")
                    .map_into_boxed_body(),
            ))
        })
    }
}
