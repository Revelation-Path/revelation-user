//! Actix-web extractors for Claims.

use std::sync::Arc;

use actix_web::{Error, FromRequest, HttpRequest, dev::Payload};
use futures_util::future::{Ready, ready};
use masterror::AppError;

use crate::Claims;

/// JWT manager for token validation.
pub trait JwtValidator: Send + Sync + 'static {
    /// Decode and validate JWT token.
    fn decode(&self, token: &str) -> Result<Claims, AppError>;
}

/// Auth configuration.
pub trait AuthConfig: Send + Sync + 'static {
    /// Cookie name for JWT token.
    fn cookie_name(&self) -> &str;
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let config = match req.app_data::<Arc<dyn AuthConfig>>() {
            Some(c) => c.clone(),
            None => return ready(Err(AppError::internal("AuthConfig missing").into()))
        };

        let jwt = match req.app_data::<Arc<dyn JwtValidator>>() {
            Some(j) => j.clone(),
            None => return ready(Err(AppError::internal("JwtValidator missing").into()))
        };

        // Try cookie first
        let token = match req.cookie(config.cookie_name()) {
            Some(c) => c.value().to_owned(),
            None => {
                // Fallback to Authorization header
                match req
                    .headers()
                    .get("Authorization")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|h| h.strip_prefix("Bearer "))
                {
                    Some(t) => t.to_owned(),
                    None => return ready(Err(AppError::unauthorized("Unauthorized").into()))
                }
            }
        };

        match jwt.decode(&token) {
            Ok(claims) => ready(Ok(claims)),
            Err(e) => ready(Err(e.into()))
        }
    }
}

/// Optional claims extractor.
#[derive(Debug, Clone)]
pub struct OptionalClaims(pub Option<Claims>);

impl FromRequest for OptionalClaims {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        match Claims::from_request(req, payload).into_inner() {
            Ok(claims) => ready(Ok(OptionalClaims(Some(claims)))),
            Err(_) => ready(Ok(OptionalClaims(None)))
        }
    }
}
