//! Axum extractors for JWT authentication.
//!
//! This module provides [`Claims`] extraction from HTTP requests
//! using the [Axum](https://crates.io/crates/axum) web framework.
//!
//! # Overview
//!
//! | Type | Purpose |
//! |------|---------|
//! | [`JwtValidator`] | Trait for JWT token decoding |
//! | [`AuthConfig`] | Trait for authentication configuration |
//! | [`OptionalClaims`] | Extractor for optional authentication |
//!
//! # Setup
//!
//! 1. Implement [`JwtValidator`] for your JWT library
//! 2. Implement [`AuthConfig`] to specify cookie name
//! 3. Add both as extensions to your router
//!
//! # Token Resolution Order
//!
//! The extractor looks for JWT tokens in this order:
//! 1. Cookie with name from [`AuthConfig::cookie_name`]
//! 2. `Authorization: Bearer <token>` header
//!
//! # Example Setup
//!
//! ```rust,ignore
//! use axum::{Router, Extension, middleware};
//! use revelation_user::{Claims, extract::{JwtValidator, AuthConfig}};
//! use std::sync::Arc;
//!
//! // 1. Implement JwtValidator
//! struct MyJwtManager {
//!     secret: String,
//! }
//!
//! impl JwtValidator for MyJwtManager {
//!     fn decode(&self, token: &str) -> Result<Claims, AppError> {
//!         // Use jsonwebtoken or similar
//!         let data = jsonwebtoken::decode::<Claims>(
//!             token,
//!             &DecodingKey::from_secret(self.secret.as_bytes()),
//!             &Validation::default(),
//!         )?;
//!         Ok(data.claims)
//!     }
//! }
//!
//! // 2. Implement AuthConfig
//! struct MyAuthConfig;
//!
//! impl AuthConfig for MyAuthConfig {
//!     fn cookie_name(&self) -> &str {
//!         "auth_token"
//!     }
//! }
//!
//! // 3. Create router with extensions
//! let app = Router::new()
//!     .route("/me", get(get_current_user))
//!     .layer(Extension(Arc::new(MyJwtManager { secret: "..." }) as Arc<dyn JwtValidator>))
//!     .layer(Extension(Arc::new(MyAuthConfig) as Arc<dyn AuthConfig>));
//! ```
//!
//! # Handler Examples
//!
//! ```rust,ignore
//! use axum::Json;
//! use revelation_user::{Claims, OptionalClaims, RUserPublic};
//!
//! // Required authentication
//! async fn get_current_user(claims: Claims) -> Json<String> {
//!     Json(format!("User ID: {}", claims.user_id()))
//! }
//!
//! // Optional authentication
//! async fn get_profile(OptionalClaims(claims): OptionalClaims) -> Json<String> {
//!     match claims {
//!         Some(c) => Json(format!("Welcome back, {}", c.user_id())),
//!         None => Json("Welcome, guest!".into()),
//!     }
//! }
//!
//! // Admin-only endpoint
//! async fn admin_dashboard(claims: Claims) -> Result<Json<String>, AppError> {
//!     if !claims.is_admin() {
//!         return Err(AppError::forbidden("Admin access required"));
//!     }
//!     Ok(Json("Admin dashboard".into()))
//! }
//! ```
//!
//! [`Claims`]: crate::Claims

use std::sync::Arc;

use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    extract::CookieJar,
    headers::{Authorization, authorization::Bearer}
};
use masterror::AppError;

use crate::Claims;

/// Trait for JWT token validation and decoding.
///
/// Implement this trait to integrate your JWT library
/// (e.g., `jsonwebtoken`, `jwt-simple`) with the Claims extractor.
///
/// # Thread Safety
///
/// Implementations must be `Send + Sync` for use with
/// async handlers across multiple threads.
///
/// # Example Implementation
///
/// ```rust,ignore
/// use revelation_user::{Claims, extract::JwtValidator};
/// use masterror::AppError;
/// use jsonwebtoken::{decode, DecodingKey, Validation};
///
/// pub struct JwtManager {
///     decoding_key: DecodingKey,
///     validation: Validation,
/// }
///
/// impl JwtManager {
///     pub fn new(secret: &str) -> Self {
///         Self {
///             decoding_key: DecodingKey::from_secret(secret.as_bytes()),
///             validation: Validation::default(),
///         }
///     }
/// }
///
/// impl JwtValidator for JwtManager {
///     fn decode(&self, token: &str) -> Result<Claims, AppError> {
///         decode::<Claims>(token, &self.decoding_key, &self.validation)
///             .map(|data| data.claims)
///             .map_err(|e| AppError::unauthorized(format!("Invalid token: {}", e)))
///     }
/// }
/// ```
pub trait JwtValidator: Send + Sync {
    /// Decode and validate a JWT token string.
    ///
    /// # Arguments
    ///
    /// * `token` - Raw JWT token string (without "Bearer " prefix)
    ///
    /// # Returns
    ///
    /// - `Ok(Claims)` - Successfully decoded claims
    /// - `Err(AppError)` - Token invalid, expired, or malformed
    ///
    /// # Errors
    ///
    /// Should return appropriate errors for:
    /// - Expired tokens
    /// - Invalid signatures
    /// - Malformed tokens
    /// - Missing required claims
    fn decode(&self, token: &str) -> Result<Claims, AppError>;
}

/// Trait for authentication configuration.
///
/// Provides configuration values needed by the Claims extractor.
///
/// # Thread Safety
///
/// Implementations must be `Send + Sync` for use across
/// async handlers.
///
/// # Example Implementation
///
/// ```rust,ignore
/// use revelation_user::extract::AuthConfig;
///
/// pub struct AppAuthConfig {
///     cookie_name: String,
/// }
///
/// impl AppAuthConfig {
///     pub fn new(cookie_name: impl Into<String>) -> Self {
///         Self { cookie_name: cookie_name.into() }
///     }
/// }
///
/// impl AuthConfig for AppAuthConfig {
///     fn cookie_name(&self) -> &str {
///         &self.cookie_name
///     }
/// }
///
/// // Usage
/// let config = AppAuthConfig::new("auth_token");
/// ```
pub trait AuthConfig: Send + Sync {
    /// Returns the cookie name used for JWT storage.
    ///
    /// The extractor will look for a cookie with this name
    /// before falling back to the Authorization header.
    ///
    /// # Common Values
    ///
    /// - `"auth_token"` - Generic auth cookie
    /// - `"jwt"` - JWT-specific
    /// - `"session"` - Session-style naming
    fn cookie_name(&self) -> &str;
}

/// Axum extractor implementation for [`Claims`].
///
/// Automatically extracts and validates JWT tokens from requests.
///
/// # Resolution Order
///
/// 1. Cookie (name from [`AuthConfig::cookie_name`])
/// 2. `Authorization: Bearer <token>` header
///
/// # Errors
///
/// Returns [`AppError`] for:
/// - Missing [`AuthConfig`] extension - Internal error
/// - Missing [`JwtValidator`] extension - Internal error
/// - No token found - Unauthorized
/// - Invalid token - Unauthorized (from validator)
///
/// # Example
///
/// ```rust,ignore
/// use axum::Json;
/// use revelation_user::Claims;
///
/// async fn handler(claims: Claims) -> Json<String> {
///     // claims is automatically extracted and validated
///     Json(format!("Hello {}", claims.user_id()))
/// }
/// ```
///
/// [`Claims`]: crate::Claims
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract dependencies from extensions
        let (config, jwt): (Arc<dyn AuthConfig>, Arc<dyn JwtValidator>) = {
            let ex = &parts.extensions;

            let config = ex
                .get::<Arc<dyn AuthConfig>>()
                .cloned()
                .ok_or_else(|| AppError::internal("AuthConfig not configured"))?;

            let jwt = ex
                .get::<Arc<dyn JwtValidator>>()
                .cloned()
                .ok_or_else(|| AppError::internal("JwtValidator not configured"))?;

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
                .ok_or_else(|| AppError::unauthorized("Authentication required"))?
        };

        jwt.decode(&token)
    }
}

/// Optional claims extractor for endpoints with optional authentication.
///
/// Unlike direct [`Claims`] extraction which rejects unauthenticated
/// requests, this wrapper allows both authenticated and anonymous access.
///
/// # Use Cases
///
/// - Public content with personalization for logged-in users
/// - Optional analytics tracking
/// - Graceful degradation when auth is unavailable
///
/// # Examples
///
/// ```rust,ignore
/// use axum::Json;
/// use revelation_user::OptionalClaims;
///
/// async fn public_with_personalization(
///     OptionalClaims(claims): OptionalClaims,
/// ) -> Json<String> {
///     match claims {
///         Some(c) => Json(format!("Welcome back, user {}", c.user_id())),
///         None => Json("Welcome, guest!".into()),
///     }
/// }
///
/// // Conditional premium content
/// async fn content(OptionalClaims(claims): OptionalClaims) -> Json<String> {
///     let is_premium = claims.as_ref().map(|c| c.is_premium()).unwrap_or(false);
///
///     if is_premium {
///         Json("Premium content here".into())
///     } else {
///         Json("Basic content only".into())
///     }
/// }
/// ```
///
/// [`Claims`]: crate::Claims
#[derive(Debug, Clone)]
pub struct OptionalClaims(pub Option<Claims>);

impl OptionalClaims {
    /// Returns the inner claims if present.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let OptionalClaims(claims) = optional_claims;
    /// if let Some(c) = claims {
    ///     println!("User: {}", c.user_id());
    /// }
    /// ```
    #[must_use]
    pub fn into_inner(self) -> Option<Claims> {
        self.0
    }

    /// Returns a reference to the inner claims if present.
    #[must_use]
    pub const fn as_ref(&self) -> Option<&Claims> {
        self.0.as_ref()
    }

    /// Returns `true` if claims are present.
    #[must_use]
    pub const fn is_authenticated(&self) -> bool {
        self.0.is_some()
    }
}

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
