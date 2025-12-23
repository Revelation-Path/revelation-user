//! Framework-specific extractors for authentication.
//!
//! This module provides JWT/session extractors for web frameworks,
//! allowing you to extract [`Claims`] from incoming HTTP requests.
//!
//! # Feature Flags
//!
//! | Feature | Framework | Description |
//! |---------|-----------|-------------|
//! | `axum` | [Axum](https://crates.io/crates/axum) | Tower-based async framework |
//! | `actix` | [Actix-web](https://crates.io/crates/actix-web) | Actor-based async framework |
//!
//! **Note**: These features are mutually exclusive. If both are enabled
//! (e.g., via `--all-features`), `axum` takes precedence.
//!
//! # Authentication Flow
//!
//! The extractors check for JWT tokens in this order:
//! 1. Cookie (name configured via [`AuthConfig`])
//! 2. `Authorization: Bearer <token>` header
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                    HTTP Request                         │
//! │                                                         │
//! │  Cookie: jwt_token=eyJ...                               │
//! │  Authorization: Bearer eyJ...                           │
//! │                                                         │
//! └─────────────────────┬───────────────────────────────────┘
//!                       │
//!                       ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │                  Claims Extractor                       │
//! │                                                         │
//! │  1. Check cookie (configured name)                      │
//! │  2. Fallback to Authorization header                    │
//! │  3. Decode JWT via JwtValidator                         │
//! │                                                         │
//! └─────────────────────┬───────────────────────────────────┘
//!                       │
//!                       ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │                     Claims                              │
//! │                                                         │
//! │  user_id: Uuid                                          │
//! │  exp: DateTime<Utc>                                     │
//! │  role: RUserRole                                        │
//! │                                                         │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! # Setup Requirements
//!
//! Both frameworks require these traits to be implemented and
//! injected as application state/extensions:
//!
//! - [`JwtValidator`] - Decodes and validates JWT tokens
//! - [`AuthConfig`] - Provides cookie name for JWT storage
//!
//! # Axum Example
//!
//! ```rust,ignore
//! use axum::{Router, Json, routing::get, Extension};
//! use revelation_user::{Claims, extract::JwtValidator};
//! use std::sync::Arc;
//!
//! // Implement JwtValidator for your JWT manager
//! struct MyJwtManager { /* ... */ }
//!
//! impl JwtValidator for MyJwtManager {
//!     fn decode(&self, token: &str) -> Result<Claims, AppError> {
//!         // Decode JWT token...
//!     }
//! }
//!
//! // Handler with Claims extraction
//! async fn protected(claims: Claims) -> Json<String> {
//!     Json(format!("Hello user {}", claims.user_id()))
//! }
//!
//! // Router setup
//! let app = Router::new()
//!     .route("/protected", get(protected))
//!     .layer(Extension(Arc::new(MyJwtManager {}) as Arc<dyn JwtValidator>))
//!     .layer(Extension(Arc::new(MyAuthConfig {}) as Arc<dyn AuthConfig>));
//! ```
//!
//! # Actix-web Example
//!
//! ```rust,ignore
//! use actix_web::{web, App, HttpServer, HttpResponse};
//! use revelation_user::{Claims, extract::JwtValidator};
//! use std::sync::Arc;
//!
//! async fn protected(claims: Claims) -> HttpResponse {
//!     HttpResponse::Ok().json(claims.user_id())
//! }
//!
//! HttpServer::new(|| {
//!     App::new()
//!         .app_data(Arc::new(MyJwtManager {}) as Arc<dyn JwtValidator>)
//!         .app_data(Arc::new(MyAuthConfig {}) as Arc<dyn AuthConfig>)
//!         .route("/protected", web::get().to(protected))
//! });
//! ```
//!
//! # Optional Claims
//!
//! Use [`OptionalClaims`] when authentication is optional:
//!
//! ```rust,ignore
//! use revelation_user::OptionalClaims;
//!
//! async fn maybe_protected(OptionalClaims(claims): OptionalClaims) -> Json<String> {
//!     match claims {
//!         Some(c) => Json(format!("Hello {}", c.user_id())),
//!         None => Json("Hello anonymous".into()),
//!     }
//! }
//! ```
//!
//! [`Claims`]: crate::Claims
//! [`AuthConfig`]: self::AuthConfig
//! [`JwtValidator`]: self::JwtValidator
//! [`OptionalClaims`]: self::OptionalClaims

// When both features enabled, axum takes precedence
#[cfg(feature = "axum")]
mod axum_extract;
#[cfg(feature = "axum")]
pub use axum_extract::*;

#[cfg(all(feature = "actix", not(feature = "axum")))]
mod actix_extract;
#[cfg(all(feature = "actix", not(feature = "axum")))]
pub use actix_extract::*;
