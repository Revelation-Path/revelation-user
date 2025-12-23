//! Data Transfer Objects for API operations.
//!
//! This module provides request/response types for user-related
//! API endpoints. All DTOs support validation via the `validator` crate.
//!
//! # Overview
//!
//! | DTO | Purpose |
//! |-----|---------|
//! | [`CreateUserRequest`] | Create a new user |
//! | [`UpdateProfileRequest`] | Update user profile |
//! | [`BindTelegram`] | Bind Telegram account |
//! | [`BindEmail`] | Bind email address |
//! | [`BindPhone`] | Bind phone number |
//!
//! # Validation
//!
//! All DTOs derive `Validate` from the `validator` crate:
//!
//! ```rust
//! use revelation_user::UpdateProfileRequest;
//! use validator::Validate;
//!
//! let req = UpdateProfileRequest {
//!     name:          Some("A".into()), // Too short (min 2)
//!     gender:        None,
//!     birth_date:    None,
//!     confession_id: None
//! };
//!
//! assert!(req.validate().is_err());
//! ```
//!
//! # OpenAPI
//!
//! With the `api` feature, all DTOs derive `utoipa::ToSchema`
//! for automatic OpenAPI schema generation.

mod bind;
mod create;
mod update;

pub use bind::*;
pub use create::*;
pub use update::*;
