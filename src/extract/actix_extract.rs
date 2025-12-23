//! Actix-web extractors for JWT authentication.
//!
//! This module provides [`Claims`] extraction from HTTP requests
//! using the [Actix-web](https://crates.io/crates/actix-web) framework.
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
//! 3. Add both as app data in your HttpServer
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
//! use actix_web::{App, HttpServer, web};
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
//! // 3. Create HttpServer with app data
//! HttpServer::new(|| {
//!     App::new()
//!         .app_data(Arc::new(MyJwtManager { secret: "..." }) as Arc<dyn JwtValidator>)
//!         .app_data(Arc::new(MyAuthConfig) as Arc<dyn AuthConfig>)
//!         .route("/me", web::get().to(get_current_user))
//! })
//! .bind("127.0.0.1:8080")?
//! .run()
//! .await
//! ```
//!
//! # Handler Examples
//!
//! ```rust,ignore
//! use actix_web::{HttpResponse, Responder};
//! use revelation_user::{Claims, OptionalClaims};
//!
//! // Required authentication
//! async fn get_current_user(claims: Claims) -> impl Responder {
//!     HttpResponse::Ok().json(format!("User ID: {}", claims.user_id()))
//! }
//!
//! // Optional authentication
//! async fn get_profile(OptionalClaims(claims): OptionalClaims) -> impl Responder {
//!     match claims {
//!         Some(c) => HttpResponse::Ok().json(format!("Welcome back, {}", c.user_id())),
//!         None => HttpResponse::Ok().json("Welcome, guest!"),
//!     }
//! }
//!
//! // Admin-only endpoint
//! async fn admin_dashboard(claims: Claims) -> Result<HttpResponse, AppError> {
//!     if !claims.is_admin() {
//!         return Err(AppError::forbidden("Admin access required"));
//!     }
//!     Ok(HttpResponse::Ok().json("Admin dashboard"))
//! }
//! ```
//!
//! [`Claims`]: crate::Claims

use std::sync::Arc;

use actix_web::{Error, FromRequest, HttpRequest, dev::Payload};
use futures_util::future::{Ready, ready};
use masterror::AppError;

use crate::Claims;

/// Trait for JWT token validation and decoding.
///
/// Implement this trait to integrate your JWT library
/// (e.g., `jsonwebtoken`, `jwt-simple`) with the Claims extractor.
///
/// # Thread Safety
///
/// Implementations must be `Send + Sync + 'static` for use with
/// Actix-web's async handlers and app data.
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
pub trait JwtValidator: Send + Sync + 'static {
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
/// Implementations must be `Send + Sync + 'static` for use
/// with Actix-web's app data.
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
pub trait AuthConfig: Send + Sync + 'static {
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

/// Actix-web extractor implementation for [`Claims`].
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
/// Returns error for:
/// - Missing [`AuthConfig`] app data - Internal error
/// - Missing [`JwtValidator`] app data - Internal error
/// - No token found - Unauthorized
/// - Invalid token - Unauthorized (from validator)
///
/// # Example
///
/// ```rust,ignore
/// use actix_web::HttpResponse;
/// use revelation_user::Claims;
///
/// async fn handler(claims: Claims) -> HttpResponse {
///     // claims is automatically extracted and validated
///     HttpResponse::Ok().json(claims.user_id())
/// }
/// ```
///
/// [`Claims`]: crate::Claims
impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Get config from app data
        let config = match req.app_data::<Arc<dyn AuthConfig>>() {
            Some(c) => c.clone(),
            None => return ready(Err(AppError::internal("AuthConfig not configured").into()))
        };

        // Get JWT validator from app data
        let jwt = match req.app_data::<Arc<dyn JwtValidator>>() {
            Some(j) => j.clone(),
            None => return ready(Err(AppError::internal("JwtValidator not configured").into()))
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
                    None => {
                        return ready(
                            Err(AppError::unauthorized("Authentication required").into())
                        );
                    }
                }
            }
        };

        match jwt.decode(&token) {
            Ok(claims) => ready(Ok(claims)),
            Err(e) => ready(Err(e.into()))
        }
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
/// use actix_web::HttpResponse;
/// use revelation_user::OptionalClaims;
///
/// async fn public_with_personalization(
///     OptionalClaims(claims): OptionalClaims,
/// ) -> HttpResponse {
///     match claims {
///         Some(c) => HttpResponse::Ok().json(format!("Welcome back, user {}", c.user_id())),
///         None => HttpResponse::Ok().json("Welcome, guest!"),
///     }
/// }
///
/// // Conditional premium content
/// async fn content(OptionalClaims(claims): OptionalClaims) -> HttpResponse {
///     let is_premium = claims.as_ref().map(|c| c.is_premium()).unwrap_or(false);
///
///     if is_premium {
///         HttpResponse::Ok().json("Premium content here")
///     } else {
///         HttpResponse::Ok().json("Basic content only")
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
