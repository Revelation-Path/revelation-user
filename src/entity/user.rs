//! User entity - the core aggregate of the user domain.
//!
//! This module defines [`RUser`], the primary user representation in the
//! Revelation ecosystem. Uses `entity-derive` for DTO and repository
//! generation.
//!
//! # Generated Types
//!
//! The `Entity` derive macro generates:
//! - [`CreateRUserRequest`] - DTO for user creation
//! - [`UpdateRUserRequest`] - DTO for profile updates
//! - [`RUserResponse`] - DTO for API responses
//! - [`RUserRow`] - Database row mapping
//! - [`InsertableRUser`] - For INSERT operations
//! - [`RUserRepository`] - Async CRUD trait
//!
//! # Examples
//!
//! ```rust
//! use revelation_user::RUser;
//!
//! // From Telegram authentication
//! let user = RUser::from_telegram(123456789);
//!
//! // From email authentication
//! let user = RUser::from_email("user@example.com");
//!
//! // Empty user (for OAuth flows)
//! let user = RUser::empty();
//! ```

use chrono::{DateTime, NaiveDate, Utc};
use entity_derive::Entity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Gender;

/// Core user entity for the Revelation ecosystem.
///
/// `RUser` represents a registered user with support for multiple
/// authentication methods and optional profile data.
///
/// # Fields
///
/// | Field | Type | Create | Update | Response |
/// |-------|------|--------|--------|----------|
/// | `id` | `Uuid` | — | — | Yes |
/// | `name` | `Option<String>` | — | Yes | Yes |
/// | `gender` | `Option<Gender>` | — | Yes | Yes |
/// | `birth_date` | `Option<NaiveDate>` | — | Yes | Yes |
/// | `confession_id` | `Option<Uuid>` | — | Yes | Yes |
/// | `email` | `Option<String>` | Yes | — | Yes |
/// | `phone` | `Option<String>` | Yes | — | Yes |
/// | `telegram_id` | `Option<i64>` | Yes | — | Yes |
/// | `created_at` | `DateTime<Utc>` | — | — | Yes |
/// | `updated_at` | `DateTime<Utc>` | — | — | Yes |
#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[entity(table = "users", schema = "public", sql = "none")]
pub struct RUser {
    /// Unique user identifier (UUIDv7).
    #[id]
    pub id: Uuid,

    /// Display name (2-100 chars).
    #[field(update, response)]
    pub name: Option<String>,

    /// User's gender.
    #[field(update, response)]
    pub gender: Option<Gender>,

    /// Date of birth.
    #[field(update, response)]
    pub birth_date: Option<NaiveDate>,

    /// Reference to confession/denomination.
    #[field(update, response)]
    pub confession_id: Option<Uuid>,

    /// Verified email address.
    #[field(create, response)]
    pub email: Option<String>,

    /// Phone number in E.164 format.
    #[field(create, response)]
    pub phone: Option<String>,

    /// Telegram user ID.
    #[field(create, response)]
    pub telegram_id: Option<i64>,

    /// Creation timestamp.
    #[field(response)]
    #[auto]
    pub created_at: DateTime<Utc>,

    /// Last update timestamp.
    #[field(response)]
    #[auto]
    pub updated_at: DateTime<Utc>
}

impl RUser {
    /// Create user from Telegram authentication.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUser;
    ///
    /// let user = RUser::from_telegram(123456789);
    /// assert_eq!(user.telegram_id, Some(123456789));
    /// ```
    #[must_use]
    pub fn from_telegram(telegram_id: i64) -> Self {
        Self {
            id:            Uuid::now_v7(),
            name:          None,
            gender:        None,
            birth_date:    None,
            confession_id: None,
            email:         None,
            phone:         None,
            telegram_id:   Some(telegram_id),
            created_at:    Utc::now(),
            updated_at:    Utc::now()
        }
    }

    /// Create user from email authentication.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUser;
    ///
    /// let user = RUser::from_email("user@example.com");
    /// assert_eq!(user.email.as_deref(), Some("user@example.com"));
    /// ```
    #[must_use]
    pub fn from_email(email: impl Into<String>) -> Self {
        Self {
            id:            Uuid::now_v7(),
            name:          None,
            gender:        None,
            birth_date:    None,
            confession_id: None,
            email:         Some(email.into()),
            phone:         None,
            telegram_id:   None,
            created_at:    Utc::now(),
            updated_at:    Utc::now()
        }
    }

    /// Create user from phone authentication.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUser;
    ///
    /// let user = RUser::from_phone("+79991234567");
    /// assert!(user.phone.is_some());
    /// ```
    #[must_use]
    pub fn from_phone(phone: impl Into<String>) -> Self {
        Self {
            id:            Uuid::now_v7(),
            name:          None,
            gender:        None,
            birth_date:    None,
            confession_id: None,
            email:         None,
            phone:         Some(phone.into()),
            telegram_id:   None,
            created_at:    Utc::now(),
            updated_at:    Utc::now()
        }
    }

    /// Create empty user with only ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUser;
    ///
    /// let user = RUser::empty();
    /// assert!(user.email.is_none());
    /// ```
    #[must_use]
    pub fn empty() -> Self {
        Self {
            id:            Uuid::now_v7(),
            name:          None,
            gender:        None,
            birth_date:    None,
            confession_id: None,
            email:         None,
            phone:         None,
            telegram_id:   None,
            created_at:    Utc::now(),
            updated_at:    Utc::now()
        }
    }

    /// Create user with a specific ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUser;
    /// use uuid::Uuid;
    ///
    /// let id = Uuid::now_v7();
    /// let user = RUser::with_id(id);
    /// assert_eq!(user.id, id);
    /// ```
    #[must_use]
    pub fn with_id(id: Uuid) -> Self {
        Self {
            id,
            name: None,
            gender: None,
            birth_date: None,
            confession_id: None,
            email: None,
            phone: None,
            telegram_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_telegram_sets_telegram_id() {
        let user = RUser::from_telegram(123);
        assert_eq!(user.telegram_id, Some(123));
        assert!(user.email.is_none());
    }

    #[test]
    fn from_email_sets_email() {
        let user = RUser::from_email("test@example.com");
        assert_eq!(user.email.as_deref(), Some("test@example.com"));
        assert!(user.telegram_id.is_none());
    }

    #[test]
    fn from_phone_sets_phone() {
        let user = RUser::from_phone("+14155551234");
        assert_eq!(user.phone.as_deref(), Some("+14155551234"));
    }

    #[test]
    fn empty_has_only_id() {
        let user = RUser::empty();
        assert!(user.name.is_none());
        assert!(user.email.is_none());
        assert!(user.telegram_id.is_none());
    }

    #[test]
    fn with_id_sets_specific_id() {
        let id = Uuid::nil();
        let user = RUser::with_id(id);
        assert_eq!(user.id, id);
    }

    #[test]
    fn serializes_to_json() {
        let user = RUser::from_telegram(123);
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"telegram_id\":123"));
    }
}
