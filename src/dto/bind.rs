// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

//! Contact binding DTOs.
//!
//! This module provides request types for binding contact methods
//! (Telegram, email, phone) to existing user accounts.
//!
//! # Overview
//!
//! | DTO | Purpose | Validation |
//! |-----|---------|------------|
//! | [`BindTelegram`] | Bind Telegram account | ID ≥ 1 |
//! | [`BindEmail`] | Bind email address | Valid email format |
//! | [`BindPhone`] | Bind phone number | E.164 format regex |
//!
//! # Examples
//!
//! ```rust
//! use revelation_user::{BindEmail, BindPhone, BindTelegram};
//! use validator::Validate;
//!
//! // Telegram binding
//! let bind = BindTelegram {
//!     telegram_id: 123456789
//! };
//! assert!(bind.validate().is_ok());
//!
//! // Email binding
//! let bind = BindEmail {
//!     email: "user@example.com".into()
//! };
//! assert!(bind.validate().is_ok());
//!
//! // Phone binding
//! let bind = BindPhone {
//!     phone: "+14155551234".into()
//! };
//! assert!(bind.validate().is_ok());
//! ```

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request to bind Telegram account to user.
///
/// Used when a user wants to link their Telegram account
/// to an existing profile.
///
/// # Validation
///
/// - `telegram_id`: Must be positive (≥ 1)
///
/// # Examples
///
/// ```rust
/// use revelation_user::BindTelegram;
/// use validator::Validate;
///
/// let valid = BindTelegram {
///     telegram_id: 123456789
/// };
/// assert!(valid.validate().is_ok());
///
/// let invalid = BindTelegram {
///     telegram_id: 0
/// };
/// assert!(invalid.validate().is_err());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct BindTelegram {
    /// Telegram user ID from bot callback.
    ///
    /// Must be a positive integer (Telegram IDs start from 1).
    #[validate(range(min = 1))]
    pub telegram_id: i64
}

/// Request to bind email address to user.
///
/// Used when a user wants to add or change their email address.
/// The email should be verified before binding.
///
/// # Validation
///
/// - `email`: Must be valid email format
///
/// # Examples
///
/// ```rust
/// use revelation_user::BindEmail;
/// use validator::Validate;
///
/// let valid = BindEmail {
///     email: "user@example.com".into()
/// };
/// assert!(valid.validate().is_ok());
///
/// let invalid = BindEmail {
///     email: "not-an-email".into()
/// };
/// assert!(invalid.validate().is_err());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct BindEmail {
    /// Email address to bind.
    ///
    /// Must be a valid email format (validated by `validator` crate).
    #[validate(email)]
    pub email: String
}

/// Request to bind phone number to user.
///
/// Used when a user wants to add or change their phone number.
/// The phone should be verified (e.g., via SMS) before binding.
///
/// # Validation
///
/// - `phone`: Must match E.164 format (`+` followed by 10-15 digits)
///
/// # Examples
///
/// ```rust
/// use revelation_user::BindPhone;
/// use validator::Validate;
///
/// // Valid E.164 formats
/// let valid = BindPhone {
///     phone: "+14155551234".into()
/// };
/// assert!(valid.validate().is_ok());
///
/// let valid_ru = BindPhone {
///     phone: "+79991234567".into()
/// };
/// assert!(valid_ru.validate().is_ok());
///
/// // Invalid formats
/// let no_plus = BindPhone {
///     phone: "14155551234".into()
/// };
/// assert!(no_plus.validate().is_err());
///
/// let too_short = BindPhone {
///     phone: "+1234".into()
/// };
/// assert!(too_short.validate().is_err());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct BindPhone {
    /// Phone number in E.164 format.
    ///
    /// Format: `+[country code][number]`
    /// - Must start with `+`
    /// - Followed by 10-15 digits
    /// - First digit after `+` must be 1-9
    ///
    /// Examples: `+14155551234`, `+442071234567`, `+79991234567`
    #[validate(regex(path = *crate::PHONE_REGEX))]
    pub phone: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telegram_validates_positive_id() {
        assert!(
            BindTelegram {
                telegram_id: 1
            }
            .validate()
            .is_ok()
        );
        assert!(
            BindTelegram {
                telegram_id: 0
            }
            .validate()
            .is_err()
        );
        assert!(
            BindTelegram {
                telegram_id: -1
            }
            .validate()
            .is_err()
        );
    }

    #[test]
    fn email_validates_format() {
        assert!(
            BindEmail {
                email: "test@example.com".into()
            }
            .validate()
            .is_ok()
        );
        assert!(
            BindEmail {
                email: "invalid".into()
            }
            .validate()
            .is_err()
        );
        assert!(
            BindEmail {
                email: "@no-local.com".into()
            }
            .validate()
            .is_err()
        );
    }

    #[test]
    fn phone_validates_e164_format() {
        // Valid
        assert!(
            BindPhone {
                phone: "+14155551234".into()
            }
            .validate()
            .is_ok()
        );
        assert!(
            BindPhone {
                phone: "+79991234567".into()
            }
            .validate()
            .is_ok()
        );
        assert!(
            BindPhone {
                phone: "+442071234567".into()
            }
            .validate()
            .is_ok()
        );

        // Invalid
        assert!(
            BindPhone {
                phone: "14155551234".into()
            }
            .validate()
            .is_err()
        );
        assert!(
            BindPhone {
                phone: "+1234".into()
            }
            .validate()
            .is_err()
        );
        assert!(
            BindPhone {
                phone: "+0123456789".into()
            }
            .validate()
            .is_err()
        );
    }
}
