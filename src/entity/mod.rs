//! Core domain entities for user management.
//!
//! This module contains the fundamental types that represent users
//! and authentication data in the system.
//!
//! # Overview
//!
//! The entity module provides two main types:
//!
//! - [`RUser`] - The core user aggregate containing all user data
//! - [`Claims`] - JWT claims for authentication tokens
//!
//! # User Entity
//!
//! [`RUser`] is the primary user representation, designed with:
//!
//! - **Builder pattern** via [`bon`] for flexible construction
//! - **Preset constructors** for common authentication flows
//! - **Optional fields** for progressive profile completion
//! - **Timestamps** for auditing
//!
//! ```rust
//! use revelation_user::RUser;
//!
//! // Quick creation from Telegram
//! let user = RUser::from_telegram(123456789);
//!
//! // Full builder access
//! let user = RUser::builder()
//!     .id(uuid::Uuid::now_v7())
//!     .name("John Doe")
//!     .email("john@example.com")
//!     .telegram_id(123456789)
//!     .build();
//! ```
//!
//! # JWT Claims
//!
//! [`Claims`] represents the payload of JWT tokens used for authentication:
//!
//! ```rust
//! use chrono::Utc;
//! use revelation_user::{Claims, RUserRole};
//! use uuid::Uuid;
//!
//! // Token expires in 1 hour
//! let exp = (Utc::now().timestamp() + 3600) as usize;
//!
//! let claims = Claims::new(Uuid::now_v7(), RUserRole::User, exp);
//!
//! assert!(!claims.is_expired());
//! ```
//!
//! # Feature Flags
//!
//! - `db`: Enables `sqlx::FromRow` derive for database integration
//! - `api`: Enables `utoipa::ToSchema` for OpenAPI generation

mod claims;
mod user;

pub use claims::*;
pub use user::*;
