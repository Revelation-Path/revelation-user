// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

//! Core domain entities for user management.
//!
//! This module contains the fundamental types that represent users
//! and authentication data in the system.
//!
//! # Overview
//!
//! The entity module provides:
//!
//! - [`RUser`] - The core user aggregate
//! - [`Claims`] - JWT claims for authentication tokens
//!
//! # Generated Types (via entity-derive)
//!
//! - [`CreateRUserRequest`] - DTO for user creation
//! - [`UpdateRUserRequest`] - DTO for profile updates
//! - [`RUserResponse`] - DTO for API responses
//!
//! # User Entity
//!
//! [`RUser`] is the primary user representation, designed with:
//!
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
//! // From email
//! let user = RUser::from_email("john@example.com");
//!
//! // Empty user for OAuth
//! let user = RUser::empty();
//! ```
//!
//! # JWT Claims
//!
//! [`Claims`] represents the payload of JWT tokens:
//!
//! ```rust
//! use chrono::Utc;
//! use revelation_user::{Claims, RUserRole};
//! use uuid::Uuid;
//!
//! let exp = (Utc::now().timestamp() + 3600) as usize;
//! let claims = Claims::new(Uuid::now_v7(), RUserRole::User, exp);
//!
//! assert!(!claims.is_expired());
//! ```
//!
//! # Feature Flags
//!
//! - `postgres`: Enables PostgreSQL repository implementation
//! - `api`: Enables OpenAPI schema generation
//! - `validate`: Enables validation derives

mod claims;
mod user;

pub use claims::*;
pub use user::*;
