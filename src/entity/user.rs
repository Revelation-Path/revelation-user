//! User entity - the core aggregate of the user domain.
//!
//! This module defines [`RUser`], the primary user representation in the
//! Revelation ecosystem. It provides a flexible, type-safe approach to
//! user management with support for multiple authentication methods.
//!
//! # Design Philosophy
//!
//! The user entity follows these principles:
//!
//! 1. **Minimal required data**: Only `id` is required; other fields are
//!    optional
//! 2. **Progressive completion**: Users can start with just an auth method and
//!    fill profile later
//! 3. **Builder pattern**: Flexible construction with compile-time safety
//! 4. **Preset constructors**: Quick creation for common authentication flows
//!
//! # Construction
//!
//! ## Preset Constructors
//!
//! For common authentication flows, use the preset constructors:
//!
//! ```rust
//! use revelation_user::RUser;
//!
//! // Telegram authentication
//! let user = RUser::from_telegram(123456789);
//!
//! // Email authentication
//! let user = RUser::from_email("user@example.com");
//!
//! // Phone authentication
//! let user = RUser::from_phone("+14155551234");
//!
//! // Empty user (for OAuth flows)
//! let user = RUser::empty();
//! ```
//!
//! ## Builder Pattern
//!
//! For full control, use the builder:
//!
//! ```rust
//! use chrono::NaiveDate;
//! use revelation_user::{Gender, RUser};
//!
//! let user = RUser::builder()
//!     .id(uuid::Uuid::now_v7())
//!     .name("John Doe")
//!     .email("john@example.com")
//!     .gender(Gender::Male)
//!     .birth_date(NaiveDate::from_ymd_opt(1990, 1, 15).unwrap())
//!     .telegram_id(123456789)
//!     .build();
//! ```
//!
//! # Serialization
//!
//! The user entity supports serde serialization:
//!
//! ```rust
//! use revelation_user::RUser;
//!
//! let user = RUser::from_telegram(123456789);
//! let json = serde_json::to_string(&user).unwrap();
//! ```
//!
//! # Database Integration
//!
//! With the `db` feature, [`RUser`] derives `sqlx::FromRow`:
//!
//! ```rust,ignore
//! let user: RUser = sqlx::query_as("SELECT * FROM users WHERE id = $1")
//!     .bind(user_id)
//!     .fetch_one(&pool)
//!     .await?;
//! ```

use chrono::{DateTime, NaiveDate, Utc};
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
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `id` | `Uuid` | Unique identifier (UUIDv7 recommended) |
/// | `name` | `Option<String>` | Display name (2-100 chars) |
/// | `gender` | `Option<Gender>` | User's gender |
/// | `birth_date` | `Option<NaiveDate>` | Date of birth |
/// | `confession_id` | `Option<Uuid>` | Reference to confession/denomination |
/// | `email` | `Option<String>` | Verified email address |
/// | `phone` | `Option<String>` | Phone in E.164 format |
/// | `telegram_id` | `Option<i64>` | Telegram user ID |
/// | `created_at` | `DateTime<Utc>` | Creation timestamp |
/// | `updated_at` | `DateTime<Utc>` | Last update timestamp |
///
/// # Examples
///
/// ```rust
/// use revelation_user::RUser;
///
/// // From Telegram login
/// let user = RUser::from_telegram(123456789);
/// assert!(user.telegram_id.is_some());
/// assert!(user.email.is_none());
///
/// // Custom builder
/// let user = RUser::builder()
///     .id(uuid::Uuid::now_v7())
///     .name("John Doe")
///     .email("john@example.com")
///     .build();
///
/// assert_eq!(user.name.as_deref(), Some("John Doe"));
/// ```
///
/// # Database Usage
///
/// With the `db` feature enabled:
///
/// ```rust,ignore
/// use sqlx::PgPool;
/// use revelation_user::RUser;
///
/// async fn get_user(pool: &PgPool, id: Uuid) -> Result<RUser, sqlx::Error> {
///     sqlx::query_as("SELECT * FROM users WHERE id = $1")
///         .bind(id)
///         .fetch_one(pool)
///         .await
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, bon::Builder)]
#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
#[builder(on(String, into))]
pub struct RUser {
    /// Unique user identifier.
    ///
    /// UUIDv7 is recommended for sortability and performance.
    /// Generated automatically by preset constructors.
    pub id: Uuid,

    /// Display name shown in the UI.
    ///
    /// When set, should be 2-100 characters.
    /// Validated by [`UpdateProfileRequest`](crate::UpdateProfileRequest).
    pub name: Option<String>,

    /// User's gender for personalization.
    ///
    /// Used for UI customization and analytics.
    pub gender: Option<Gender>,

    /// Date of birth.
    ///
    /// Used for age-related features and verification.
    #[cfg_attr(feature = "db", sqlx(default))]
    pub birth_date: Option<NaiveDate>,

    /// Reference to user's confession or denomination.
    ///
    /// Links to a separate confession entity in the system.
    #[cfg_attr(feature = "db", sqlx(default))]
    pub confession_id: Option<Uuid>,

    /// Verified email address.
    ///
    /// Set when user authenticates via email or binds an email.
    pub email: Option<String>,

    /// Phone number in E.164 format.
    ///
    /// Example: `+14155551234`
    ///
    /// Validated by [`PHONE_REGEX`](crate::PHONE_REGEX).
    pub phone: Option<String>,

    /// Telegram user ID.
    ///
    /// Set when user authenticates via Telegram bot or widget.
    /// This is the numeric ID, not the username.
    pub telegram_id: Option<i64>,

    /// Timestamp when the user was created.
    ///
    /// Automatically set to current time by preset constructors.
    #[builder(default = Utc::now())]
    pub created_at: DateTime<Utc>,

    /// Timestamp of the last profile update.
    ///
    /// Should be updated when any user data changes.
    #[builder(default = Utc::now())]
    pub updated_at: DateTime<Utc>
}

impl RUser {
    /// Create user from Telegram authentication.
    ///
    /// Initializes a new user with only the Telegram ID set.
    /// All other fields are `None` or defaults.
    ///
    /// # Arguments
    ///
    /// * `telegram_id` - The Telegram user ID from bot callback
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUser;
    ///
    /// let user = RUser::from_telegram(123456789);
    ///
    /// assert!(user.telegram_id.is_some());
    /// assert_eq!(user.telegram_id, Some(123456789));
    /// assert!(user.email.is_none());
    /// assert!(user.name.is_none());
    /// ```
    #[must_use]
    pub fn from_telegram(telegram_id: i64) -> Self {
        Self::builder()
            .id(Uuid::now_v7())
            .telegram_id(telegram_id)
            .build()
    }

    /// Create user from email authentication.
    ///
    /// Initializes a new user with only the email set.
    /// Useful for email/password or magic link authentication.
    ///
    /// # Arguments
    ///
    /// * `email` - The verified email address
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUser;
    ///
    /// let user = RUser::from_email("user@example.com");
    ///
    /// assert_eq!(user.email.as_deref(), Some("user@example.com"));
    /// assert!(user.telegram_id.is_none());
    /// ```
    #[must_use]
    pub fn from_email(email: impl Into<String>) -> Self {
        Self::builder().id(Uuid::now_v7()).email(email).build()
    }

    /// Create user from phone authentication.
    ///
    /// Initializes a new user with only the phone number set.
    /// Phone should be in E.164 format.
    ///
    /// # Arguments
    ///
    /// * `phone` - Phone number in E.164 format (e.g., `+14155551234`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUser;
    ///
    /// let user = RUser::from_phone("+79991234567");
    ///
    /// assert!(user.phone.is_some());
    /// assert!(user.email.is_none());
    /// ```
    #[must_use]
    pub fn from_phone(phone: impl Into<String>) -> Self {
        Self::builder().id(Uuid::now_v7()).phone(phone).build()
    }

    /// Create empty user with only ID.
    ///
    /// Useful for OAuth/SSO flows where the user is created first
    /// and profile data is filled in subsequent requests.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUser;
    ///
    /// let user = RUser::empty();
    ///
    /// assert!(user.email.is_none());
    /// assert!(user.telegram_id.is_none());
    /// assert!(user.name.is_none());
    /// ```
    #[must_use]
    pub fn empty() -> Self {
        Self::builder().id(Uuid::now_v7()).build()
    }

    /// Create user with a specific ID.
    ///
    /// Useful when you need to control the ID generation,
    /// such as during data migration or testing.
    ///
    /// # Arguments
    ///
    /// * `id` - The UUID to use for this user
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUser;
    /// use uuid::Uuid;
    ///
    /// let id = Uuid::now_v7();
    /// let user = RUser::with_id(id);
    ///
    /// assert_eq!(user.id, id);
    /// ```
    #[must_use]
    pub fn with_id(id: Uuid) -> Self {
        Self::builder().id(id).build()
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
    fn builder_allows_all_fields() {
        let user = RUser::builder()
            .id(Uuid::nil())
            .name("Test")
            .email("test@test.com")
            .gender(Gender::Male)
            .telegram_id(123)
            .build();

        assert_eq!(user.name.as_deref(), Some("Test"));
        assert_eq!(user.gender, Some(Gender::Male));
    }

    #[test]
    fn empty_has_only_id() {
        let user = RUser::empty();
        assert!(user.name.is_none());
        assert!(user.email.is_none());
        assert!(user.telegram_id.is_none());
    }

    #[test]
    fn serializes_to_json() {
        let user = RUser::from_telegram(123);
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"telegram_id\":123"));
    }
}
