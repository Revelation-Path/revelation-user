//! Axum extractors for Claims.

use std::sync::Arc;

use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    extract::CookieJar,
    headers::{Authorization, authorization::Bearer}
};
use masterror::AppError;

use crate::Claims;

/// JWT manager for token validation.
pub trait JwtValidator: Send + Sync {
    /// Decode and validate JWT token.
    fn decode(&self, token: &str) -> Result<Claims, AppError>;
}

/// Auth configuration.
pub trait AuthConfig: Send + Sync {
    /// Cookie name for JWT token.
    fn cookie_name(&self) -> &str;
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let (config, jwt): (Arc<dyn AuthConfig>, Arc<dyn JwtValidator>) = {
            let ex = &parts.extensions;

            let config = ex
                .get::<Arc<dyn AuthConfig>>()
                .cloned()
                .ok_or_else(|| AppError::internal("AuthConfig missing"))?;

            let jwt = ex
                .get::<Arc<dyn JwtValidator>>()
                .cloned()
                .ok_or_else(|| AppError::internal("JwtValidator missing"))?;

            (config, jwt)
        };

        // Try cookie first
        let jwt_opt = parts
            .extract::<CookieJar>()
            .await
            .ok()
            .and_then(|jar| jar.get(config.cookie_name()).map(|c| c.value().to_owned()));

        // Fallback to Authorization header
        let token = match jwt_opt {
            Some(v) => v,
            None => parts
                .extract::<TypedHeader<Authorization<Bearer>>>()
                .await
                .ok()
                .map(|TypedHeader(Authorization(b))| b.token().to_owned())
                .ok_or_else(|| AppError::unauthorized("Unauthorized"))?
        };

        jwt.decode(&token)
    }
}

/// Optional claims extractor.
///
/// Returns `None` if no valid token provided.
#[derive(Debug, Clone)]
pub struct OptionalClaims(pub Option<Claims>);

impl<S> FromRequestParts<S> for OptionalClaims
where
    S: Send + Sync
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Claims::from_request_parts(parts, state).await {
            Ok(claims) => Ok(OptionalClaims(Some(claims))),
            Err(_) => Ok(OptionalClaims(None))
        }
    }
}
