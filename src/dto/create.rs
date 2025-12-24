// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

//! User creation DTOs.
//!
//! This module provides request types for creating new users.
//!
//! # Overview
//!
//! [`CreateUserRequest`] is used when creating a user from
//! an authentication event (Telegram login, email signup, etc.).
//!
//! # Examples
//!
//! ```rust
//! use revelation_user::CreateUserRequest;
//!
//! // From Telegram authentication
//! let req = CreateUserRequest::telegram(123456789);
//! assert!(req.telegram_id.is_some());
//!
//! // From email authentication
//! let req = CreateUserRequest::email("user@example.com");
//! assert!(req.email.is_some());
//! ```

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Request to create a new user.
///
/// Contains the minimal data required to create a user record.
/// Additional profile data can be added later via [`UpdateProfileRequest`].
///
/// # Fields
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `id` | `Uuid` | Pre-generated user ID (auto-generated if not provided) |
/// | `telegram_id` | `Option<i64>` | Telegram user ID |
/// | `email` | `Option<String>` | Email address |
/// | `phone` | `Option<String>` | Phone number |
///
/// # Validation
///
/// - `telegram_id`: Must be positive (≥ 1)
/// - `email`: Must be valid email format
///
/// # Examples
///
/// ```rust
/// use revelation_user::CreateUserRequest;
/// use validator::Validate;
///
/// // Valid request
/// let req = CreateUserRequest::telegram(123456789);
/// assert!(req.validate().is_ok());
///
/// // Manual construction
/// let req = CreateUserRequest {
///     id:          uuid::Uuid::now_v7(),
///     telegram_id: Some(123),
///     email:       None,
///     phone:       None
/// };
/// ```
///
/// [`UpdateProfileRequest`]: crate::UpdateProfileRequest
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct CreateUserRequest {
    /// Pre-generated user ID.
    ///
    /// Defaults to a new UUIDv7 if not provided during deserialization.
    #[serde(default = "Uuid::now_v7")]
    pub id: Uuid,

    /// Telegram user ID from bot callback.
    ///
    /// Must be a positive integer.
    #[validate(range(min = 1))]
    pub telegram_id: Option<i64>,

    /// Email address from email authentication.
    ///
    /// Must be a valid email format.
    #[validate(email)]
    pub email: Option<String>,

    /// Phone number from phone authentication.
    ///
    /// Should be in E.164 format (e.g., `+14155551234`).
    pub phone: Option<String>
}

impl CreateUserRequest {
    /// Create request for Telegram authentication.
    ///
    /// # Arguments
    ///
    /// * `telegram_id` - The Telegram user ID from login callback
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::CreateUserRequest;
    ///
    /// let req = CreateUserRequest::telegram(123456789);
    ///
    /// assert_eq!(req.telegram_id, Some(123456789));
    /// assert!(req.email.is_none());
    /// assert!(req.phone.is_none());
    /// ```
    #[must_use]
    pub fn telegram(telegram_id: i64) -> Self {
        Self {
            id:          Uuid::now_v7(),
            telegram_id: Some(telegram_id),
            email:       None,
            phone:       None
        }
    }

    /// Create request for email authentication.
    ///
    /// # Arguments
    ///
    /// * `email` - The verified email address
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::CreateUserRequest;
    ///
    /// let req = CreateUserRequest::email("user@example.com");
    ///
    /// assert_eq!(req.email.as_deref(), Some("user@example.com"));
    /// assert!(req.telegram_id.is_none());
    /// ```
    #[must_use]
    pub fn email(email: impl Into<String>) -> Self {
        Self {
            id:          Uuid::now_v7(),
            telegram_id: None,
            email:       Some(email.into()),
            phone:       None
        }
    }

    /// Create request for phone authentication.
    ///
    /// # Arguments
    ///
    /// * `phone` - The phone number in E.164 format
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::CreateUserRequest;
    ///
    /// let req = CreateUserRequest::phone("+14155551234");
    ///
    /// assert_eq!(req.phone.as_deref(), Some("+14155551234"));
    /// ```
    #[must_use]
    pub fn phone(phone: impl Into<String>) -> Self {
        Self {
            id:          Uuid::now_v7(),
            telegram_id: None,
            email:       None,
            phone:       Some(phone.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telegram_constructor() {
        let req = CreateUserRequest::telegram(123);
        assert_eq!(req.telegram_id, Some(123));
        assert!(req.email.is_none());
    }

    #[test]
    fn email_constructor() {
        let req = CreateUserRequest::email("test@test.com");
        assert_eq!(req.email.as_deref(), Some("test@test.com"));
    }

    #[test]
    fn validates_telegram_id_positive() {
        let req = CreateUserRequest {
            id:          Uuid::nil(),
            telegram_id: Some(0),
            email:       None,
            phone:       None
        };
        assert!(req.validate().is_err());

        let req = CreateUserRequest::telegram(1);
        assert!(req.validate().is_ok());
    }

    #[test]
    fn validates_email_format() {
        let req = CreateUserRequest {
            id:          Uuid::nil(),
            telegram_id: None,
            email:       Some("invalid".into()),
            phone:       None
        };
        assert!(req.validate().is_err());

        let req = CreateUserRequest::email("valid@example.com");
        assert!(req.validate().is_ok());
    }

    #[test]
    fn phone_constructor() {
        let req = CreateUserRequest::phone("+14155551234");
        assert_eq!(req.phone.as_deref(), Some("+14155551234"));
        assert!(req.telegram_id.is_none());
        assert!(req.email.is_none());
    }
}
