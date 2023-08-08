use actix_web::{
    dev::Payload,
    error::{Error as ActixWebError, ErrorUnauthorized},
    http, web, FromRequest, HttpRequest,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde_json::json;
use std::{future::Future, pin::Pin};

use crate::models::{AppState, TokenClaims};

pub struct AuthenticationGuard {
    pub user_id: String,
}

impl FromRequest for AuthenticationGuard {
    type Error = ActixWebError;
    // type Future = Ready<Result<Self, Self::Error>>;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>; // Box::pin

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extracted_token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .and_then(|header| header.to_str().ok())
                    .map(|h| h.trim_start_matches("Bearer ").to_string())
            });

        let app_data = req.app_data::<web::Data<AppState>>().cloned();

        Box::pin(async move {
            let token = extracted_token.ok_or_else(|| {
                ErrorUnauthorized(json!({
                    "status": "fail",
                    "message": "You are not logged in, please provide token"
                }))
            })?;

            let data = app_data.ok_or_else(|| {
                ErrorUnauthorized(json!({"status": "fail", "message": "Internal Server Error"}))
            })?;

            let token_data = decode::<TokenClaims>(
                &token,
                &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
                &Validation::new(Algorithm::HS256),
            )
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    ErrorUnauthorized(json!({"status": "fail", "message": "Token has expired"}))
                }
                _ => ErrorUnauthorized(json!({"status": "fail", "message": "Invalid token"})),
            })
            .unwrap();

            let vec = data.db.lock().await;

            match vec
                .iter()
                .find(|user| user.id.as_ref() == Some(&token_data.claims.sub))
            {
                Some(_) => Ok(AuthenticationGuard {
                    user_id: token_data.claims.sub,
                }),
                None => Err(ErrorUnauthorized(
                    json!({"status": "fail", "message": "User belonging to this token no longer exists"}),
                )),
            }
        })
    }
}
