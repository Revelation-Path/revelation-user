//! User projections for different contexts.
//!
//! Projections are read-only views of the [`RUser`] entity,
//! optimized for specific use cases. They provide type-safe
//! data filtering to prevent accidental exposure of sensitive
//! information.
//!
//! # Overview
//!
//! | Projection | Purpose | Excludes |
//! |------------|---------|----------|
//! | [`RUserPublic`] | API responses | email, phone, telegram_id |
//! | [`RUserAuth`] | JWT/session context | personal data, includes role |
//!
//! # Design Philosophy
//!
//! Instead of manually selecting fields for each API response,
//! projections provide compile-time guarantees that sensitive
//! data cannot leak:
//!
//! ```rust
//! use revelation_user::{RUser, RUserPublic};
//!
//! let user = RUser::builder()
//!     .id(uuid::Uuid::now_v7())
//!     .email("secret@example.com")
//!     .telegram_id(123456789)
//!     .build();
//!
//! // Convert to public projection - sensitive fields excluded
//! let public: RUserPublic = user.into();
//!
//! // public.email - doesn't exist!
//! // public.telegram_id - doesn't exist!
//! ```
//!
//! # When to Use
//!
//! ## [`RUserPublic`]
//!
//! Use for any data returned to clients:
//! - User profile endpoints
//! - User search results
//! - Social features (followers, friends)
//!
//! ## [`RUserAuth`]
//!
//! Use for authentication/authorization contexts:
//! - JWT token payload
//! - Session storage
//! - Permission checks
//!
//! # Examples
//!
//! ## Converting from [`RUser`]
//!
//! ```rust
//! use revelation_user::{RUser, RUserAuth, RUserPublic, RUserRole};
//!
//! let user = RUser::from_telegram(123456789);
//!
//! // Owned conversion
//! let public: RUserPublic = user.clone().into();
//!
//! // Reference conversion
//! let public_ref: RUserPublic = (&user).into();
//!
//! // Auth projection with role
//! let auth = RUserAuth::from_user(&user, RUserRole::Premium);
//! ```
//!
//! ## In API Handlers
//!
//! ```rust,ignore
//! use axum::Json;
//! use revelation_user::{RUser, RUserPublic};
//!
//! async fn get_user(user: RUser) -> Json<RUserPublic> {
//!     // Automatically converts, excluding sensitive fields
//!     Json(user.into())
//! }
//! ```
//!
//! [`RUser`]: crate::RUser

mod auth;
mod public;

pub use auth::*;
pub use public::*;
