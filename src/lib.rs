//! # revelation-user
//!
//! A professional user domain crate for the Revelation ecosystem.
//!
//! This crate provides a complete user management solution with:
//! - Type-safe user entity with builder pattern
//! - Multiple authentication methods (Telegram, email, phone)
//! - Framework-agnostic extractors for axum and actix-web
//! - Extensible architecture for application-specific user types
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! revelation-user = { version = "0.1", features = ["axum"] }
//! ```
//!
//! Create users with the fluent builder API:
//!
//! ```rust
//! use revelation_user::RUser;
//!
//! // From Telegram authentication
//! let user = RUser::from_telegram(123456789);
//!
//! // From email authentication
//! let user = RUser::from_email("john@example.com");
//! ```
//!
//! ## Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `db` | Database support via sqlx (PostgreSQL) |
//! | `api` | OpenAPI schema generation via utoipa |
//! | `axum` | Axum framework extractors |
//! | `actix` | Actix-web framework extractors |
//!
//! **Note**: `axum` and `actix` features are mutually exclusive.
//!
//! ## Core Types
//!
//! ### Entity
//!
//! - [`RUser`] - The core user entity with all fields
//! - [`Claims`] - JWT claims for authentication
//!
//! ### Projections
//!
//! Projections are read-only views optimized for specific contexts:
//!
//! - [`RUserPublic`] - Safe for API responses (excludes sensitive data)
//! - [`RUserAuth`] - For JWT/session context (includes role)
//!
//! ```rust
//! use revelation_user::{RUser, RUserAuth, RUserPublic, RUserRole};
//!
//! let user = RUser::from_telegram(123456789);
//!
//! // Convert to public view (safe for API)
//! let public: RUserPublic = user.clone().into();
//!
//! // Convert to auth view (for JWT)
//! let auth = RUserAuth::from_user(&user, RUserRole::User);
//! ```
//!
//! ### DTOs
//!
//! Request/response data transfer objects:
//!
//! - [`CreateUserRequest`] - Create new user
//! - [`UpdateProfileRequest`] - Update user profile
//! - [`BindTelegram`], [`BindEmail`], [`BindPhone`] - Bind contact methods
//!
//! ## Extending Users
//!
//! Use the [`extend_user!`] macro to create application-specific user types:
//!
//! ```rust,ignore
//! use revelation_user::{extend_user, RUser};
//! use uuid::Uuid;
//!
//! extend_user! {
//!     /// Corporate user with company-specific fields.
//!     pub struct CorpUser {
//!         pub company_id: Uuid,
//!
//!         #[builder(into)]
//!         pub department: String,
//!     }
//! }
//!
//! // Use with fluent API
//! let user = CorpUser::from_telegram(123456789)
//!     .name("John")
//!     .then()
//!     .company_id(Uuid::now_v7())
//!     .department("Engineering")
//!     .build();
//!
//! // Access fields transparently via Deref
//! assert!(user.telegram_id.is_some());  // RUser field
//! ```
//!
//! ## Framework Integration
//!
//! ### Axum
//!
//! ```rust,ignore
//! use axum::{Router, Json, routing::get};
//! use revelation_user::{Claims, RUserPublic};
//!
//! async fn me(claims: Claims) -> Json<RUserPublic> {
//!     // Claims extracted from JWT cookie or Authorization header
//!     todo!()
//! }
//!
//! let app = Router::new().route("/me", get(me));
//! ```
//!
//! ### Actix-web
//!
//! ```rust,ignore
//! use actix_web::{web, HttpResponse};
//! use revelation_user::{Claims, RUserPublic};
//!
//! async fn me(claims: Claims) -> HttpResponse {
//!     // Claims extracted automatically
//!     HttpResponse::Ok().json(claims.user_id())
//! }
//! ```
//!
//! ## Validation
//!
//! All DTOs support validation via the `validator` crate:
//!
//! ```rust
//! use revelation_user::UpdateProfileRequest;
//! use validator::Validate;
//!
//! let req = UpdateProfileRequest {
//!     name:          Some("J".into()), // Too short!
//!     gender:        None,
//!     birth_date:    None,
//!     confession_id: None
//! };
//!
//! assert!(req.validate().is_err());
//! ```
//!
//! ## Module Structure
//!
//! - [`entity`] - Core user entity and JWT claims
//! - [`projections`] - Read-only user views
//! - [`dto`] - Request/response data transfer objects
//! - [`extend`] - Extension macro for custom user types
//! - [`extract`] - Framework extractors (feature-gated)
//! - [`ports`] - Repository trait definitions

use std::sync::LazyLock;

use regex::Regex;

pub mod dto;
pub mod entity;
pub mod extend;
mod gender;
mod notification;
mod permissions;
pub mod projections;
mod role;

#[cfg(any(feature = "axum", feature = "actix"))]
pub mod extract;

pub mod ports;

// Re-exports for convenience
pub use dto::*;
pub use entity::*;
#[cfg(any(feature = "axum", feature = "actix"))]
pub use extract::*;
pub use gender::*;
pub use notification::*;
pub use permissions::*;
pub use projections::*;
pub use role::*;

/// E.164 international phone number format regex.
///
/// Matches phone numbers in the format `+[country code][number]`:
/// - Must start with `+`
/// - Country code: 1-3 digits (first digit non-zero)
/// - Number: 9-14 additional digits
///
/// # Examples
///
/// Valid formats:
/// - `+14155551234` (US)
/// - `+442071234567` (UK)
/// - `+79991234567` (Russia)
///
/// # Usage
///
/// ```rust
/// use revelation_user::PHONE_REGEX;
///
/// assert!(PHONE_REGEX.is_match("+14155551234"));
/// assert!(!PHONE_REGEX.is_match("14155551234")); // Missing +
/// assert!(!PHONE_REGEX.is_match("+1234")); // Too short
/// ```
pub static PHONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\+[1-9]\d{9,14}$").expect("valid phone regex"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phone_regex_accepts_valid_numbers() {
        assert!(PHONE_REGEX.is_match("+14155551234"));
        assert!(PHONE_REGEX.is_match("+442071234567"));
        assert!(PHONE_REGEX.is_match("+79991234567"));
    }

    #[test]
    fn phone_regex_rejects_invalid_numbers() {
        assert!(!PHONE_REGEX.is_match("14155551234"));
        assert!(!PHONE_REGEX.is_match("+1234"));
        assert!(!PHONE_REGEX.is_match("+0123456789"));
        assert!(!PHONE_REGEX.is_match("not a phone"));
    }
}
