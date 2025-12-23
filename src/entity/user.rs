//! User entity - the core aggregate of the user domain.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Gender;

/// Core user entity for the Revelation ecosystem.
///
/// Use the builder for flexible construction or preset methods
/// for common scenarios.
///
/// # Examples
///
/// ```
/// use revelation_user::RUser;
///
/// // From Telegram login
/// let user = RUser::from_telegram(123456789);
///
/// // Custom builder
/// let user = RUser::builder()
///     .id(uuid::Uuid::now_v7())
///     .name("John Doe")
///     .email("john@example.com")
///     .build();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, bon::Builder)]
#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
#[builder(on(String, into))]
pub struct RUser {
    /// Unique identifier (UUIDv7 recommended).
    pub id: Uuid,

    /// Display name.
    pub name: Option<String>,

    /// User's gender.
    pub gender: Option<Gender>,

    /// Date of birth.
    #[cfg_attr(feature = "db", sqlx(default))]
    pub birth_date: Option<NaiveDate>,

    /// Reference to confession/denomination.
    #[cfg_attr(feature = "db", sqlx(default))]
    pub confession_id: Option<Uuid>,

    /// Verified email address.
    pub email: Option<String>,

    /// Phone number in E.164 format.
    pub phone: Option<String>,

    /// Telegram user ID.
    pub telegram_id: Option<i64>,

    /// Creation timestamp.
    #[builder(default = Utc::now())]
    pub created_at: DateTime<Utc>,

    /// Last update timestamp.
    #[builder(default = Utc::now())]
    pub updated_at: DateTime<Utc>
}

impl RUser {
    /// Create user from Telegram authentication.
    ///
    /// # Examples
    ///
    /// ```
    /// use revelation_user::RUser;
    ///
    /// let user = RUser::from_telegram(123456789);
    /// assert!(user.telegram_id.is_some());
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
    /// # Examples
    ///
    /// ```
    /// use revelation_user::RUser;
    ///
    /// let user = RUser::from_email("user@example.com");
    /// assert_eq!(user.email.as_deref(), Some("user@example.com"));
    /// ```
    #[must_use]
    pub fn from_email(email: impl Into<String>) -> Self {
        Self::builder().id(Uuid::now_v7()).email(email).build()
    }

    /// Create user from phone authentication.
    ///
    /// # Examples
    ///
    /// ```
    /// use revelation_user::RUser;
    ///
    /// let user = RUser::from_phone("+79991234567");
    /// assert!(user.phone.is_some());
    /// ```
    #[must_use]
    pub fn from_phone(phone: impl Into<String>) -> Self {
        Self::builder().id(Uuid::now_v7()).phone(phone).build()
    }

    /// Create empty user with only ID.
    ///
    /// Useful for OAuth flows where profile is filled later.
    #[must_use]
    pub fn empty() -> Self {
        Self::builder().id(Uuid::now_v7()).build()
    }

    /// Create user with specific ID.
    #[must_use]
    pub fn with_id(id: Uuid) -> Self {
        Self::builder().id(id).build()
    }
}
