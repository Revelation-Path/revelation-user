// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

//! Public user projection for API responses.
//!
//! This module provides [`RUserPublic`], a read-only view of user data
//! that is safe to expose in API responses. Sensitive fields like
//! email, phone, and telegram_id are excluded.
//!
//! # Security
//!
//! Using projections instead of the full [`RUser`] entity ensures
//! that sensitive data cannot be accidentally serialized and sent
//! to clients:
//!
//! ```rust
//! use revelation_user::{RUser, RUserPublic};
//!
//! // User with sensitive data
//! let user = RUser::from_email("secret@example.com");
//!
//! let public: RUserPublic = user.into();
//!
//! // JSON output: {"id":"...","name":null,"gender":null}
//! // Note: email and telegram_id are NOT included
//! ```
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use revelation_user::{Gender, RUser, RUserPublic};
//!
//! let mut user = RUser::empty();
//! user.name = Some("Alice".into());
//! user.gender = Some(Gender::Female);
//!
//! let public: RUserPublic = user.into();
//! assert_eq!(public.name.as_deref(), Some("Alice"));
//! assert_eq!(public.gender, Some(Gender::Female));
//! ```
//!
//! ## Reference Conversion
//!
//! ```rust
//! use revelation_user::{RUser, RUserPublic};
//!
//! let user = RUser::from_telegram(123456);
//!
//! // Convert without consuming the user
//! let public: RUserPublic = (&user).into();
//!
//! // user is still available
//! assert!(user.telegram_id.is_some());
//! ```
//!
//! [`RUser`]: crate::RUser

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Gender, RUser};

/// Public user data safe for API responses.
///
/// This projection contains only non-sensitive user information
/// that can be safely exposed to clients.
///
/// # Fields
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `id` | `Uuid` | Unique user identifier |
/// | `name` | `Option<String>` | Display name |
/// | `gender` | `Option<Gender>` | User's gender |
///
/// # Excluded Fields
///
/// The following [`RUser`] fields are intentionally excluded:
/// - `telegram_id` - Authentication identifier
/// - `email` - Personal contact information
/// - `phone` - Personal contact information
/// - `birth_date` - Sensitive personal data
/// - `confession_id` - Religious information
/// - `created_at` - Internal metadata
///
/// # Examples
///
/// ## From Owned [`RUser`]
///
/// ```rust
/// use revelation_user::{RUser, RUserPublic};
///
/// let user = RUser::from_telegram(123456789);
/// let public: RUserPublic = user.into();
/// ```
///
/// ## From Reference
///
/// ```rust
/// use revelation_user::{RUser, RUserPublic};
///
/// let user = RUser::from_email("user@example.com");
/// let public: RUserPublic = (&user).into();
///
/// // Original user still available
/// assert!(user.email.is_some());
/// ```
///
/// ## JSON Serialization
///
/// ```rust
/// use revelation_user::{Gender, RUser, RUserPublic};
/// use uuid::Uuid;
///
/// let mut user = RUser::with_id(Uuid::nil());
/// user.name = Some("Test User".into());
/// user.gender = Some(Gender::Male);
/// user.email = Some("secret@test.com".into());
///
/// let public: RUserPublic = user.into();
/// let json = serde_json::to_string(&public).unwrap();
///
/// assert!(json.contains("Test User"));
/// assert!(!json.contains("secret@test.com"));
/// ```
///
/// [`RUser`]: crate::RUser
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct RUserPublic {
    /// Unique user identifier.
    ///
    /// This is the same UUID from the source [`RUser`].
    ///
    /// [`RUser`]: crate::RUser
    pub id: Uuid,

    /// Display name.
    ///
    /// User's chosen display name, if set. May be `None` for
    /// users who haven't completed their profile.
    pub name: Option<String>,

    /// User's gender.
    ///
    /// Optional gender information, if provided by the user.
    pub gender: Option<Gender>
}

impl From<RUser> for RUserPublic {
    /// Converts an owned [`RUser`] into [`RUserPublic`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{RUser, RUserPublic};
    ///
    /// let user = RUser::from_telegram(123456);
    /// let public: RUserPublic = user.into();
    /// ```
    ///
    /// [`RUser`]: crate::RUser
    fn from(user: RUser) -> Self {
        Self {
            id:     user.id,
            name:   user.name,
            gender: user.gender
        }
    }
}

impl From<&RUser> for RUserPublic {
    /// Converts a reference to [`RUser`] into [`RUserPublic`].
    ///
    /// This allows creating a public projection without
    /// consuming the original user.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{RUser, RUserPublic};
    ///
    /// let user = RUser::from_email("user@example.com");
    /// let public: RUserPublic = (&user).into();
    ///
    /// // user is still available
    /// assert!(user.email.is_some());
    /// ```
    ///
    /// [`RUser`]: crate::RUser
    fn from(user: &RUser) -> Self {
        Self {
            id:     user.id,
            name:   user.name.clone(),
            gender: user.gender
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_user_copies_public_fields() {
        let mut user = RUser::with_id(Uuid::nil());
        user.name = Some("Test".into());
        user.gender = Some(Gender::Male);
        user.email = Some("secret@test.com".into());
        user.telegram_id = Some(123);

        let public: RUserPublic = user.into();

        assert_eq!(public.id, Uuid::nil());
        assert_eq!(public.name.as_deref(), Some("Test"));
        assert_eq!(public.gender, Some(Gender::Male));
    }

    #[test]
    fn from_ref_preserves_original() {
        let user = RUser::from_telegram(123456);
        let _public: RUserPublic = (&user).into();

        // User still accessible
        assert_eq!(user.telegram_id, Some(123456));
    }

    #[test]
    fn serialization_excludes_sensitive_fields() {
        let mut user = RUser::with_id(Uuid::nil());
        user.email = Some("secret@test.com".into());

        let public: RUserPublic = user.into();
        let json = serde_json::to_string(&public).unwrap();

        assert!(!json.contains("secret@test.com"));
        assert!(!json.contains("telegram_id"));
    }
}
