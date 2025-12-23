//! # revelation-user
//!
//! User domain crate for the Revelation ecosystem.
//!
//! Provides user entity, authentication, and profile management
//! with support for multiple web frameworks.
//!
//! ## Features
//!
//! - `db` - Database support via sqlx (PostgreSQL)
//! - `api` - OpenAPI schema generation via utoipa
//! - `axum` - Axum framework extractors
//! - `actix` - Actix-web framework extractors
//! - `backend` - Backend utilities (masterror)
//!
//! ## Quick Start
//!
//! ```rust
//! use revelation_user::RUser;
//!
//! // From Telegram authentication
//! let user = RUser::from_telegram(123456789);
//!
//! // From email
//! let user = RUser::from_email("user@example.com");
//!
//! // Custom builder
//! let user = RUser::builder()
//!     .id(uuid::Uuid::now_v7())
//!     .name("John Doe")
//!     .email("john@example.com")
//!     .telegram_id(123456789)
//!     .build();
//! ```
//!
//! ## Projections
//!
//! ```rust
//! use revelation_user::{RUser, RUserAuth, RUserPublic, RUserRole};
//!
//! let user = RUser::from_telegram(123456789);
//!
//! // Public view (safe for API responses)
//! let public: RUserPublic = user.clone().into();
//!
//! // Auth view (for JWT/session)
//! let auth = RUserAuth::from_user(&user, RUserRole::User);
//! ```

use std::sync::LazyLock;

use regex::Regex;

mod dto;
mod entity;
mod gender;
mod notification;
mod projections;
mod role;

#[cfg(any(feature = "axum", feature = "actix"))]
pub mod extract;

pub mod ports;

// Re-exports
pub use dto::*;
pub use entity::*;
#[cfg(any(feature = "axum", feature = "actix"))]
pub use extract::*;
pub use gender::*;
pub use notification::*;
pub use projections::*;
pub use role::*;

/// E.164 phone number regex.
pub static PHONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\+[1-9]\d{9,14}$").expect("valid phone regex"));
