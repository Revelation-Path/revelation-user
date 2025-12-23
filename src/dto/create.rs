//! User creation DTOs.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Request to create a new user.
///
/// Minimal required data - profile can be filled later.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct CreateUserRequest {
    /// Pre-generated user ID (UUIDv7 recommended).
    #[serde(default = "Uuid::now_v7")]
    pub id: Uuid,

    /// Telegram user ID (if from Telegram auth).
    #[validate(range(min = 1))]
    pub telegram_id: Option<i64>,

    /// Email address (if from email auth).
    #[validate(email)]
    pub email: Option<String>,

    /// Phone number (if from phone auth).
    pub phone: Option<String>
}

impl CreateUserRequest {
    /// Create request for Telegram authentication.
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
    #[must_use]
    pub fn email(email: impl Into<String>) -> Self {
        Self {
            id:          Uuid::now_v7(),
            telegram_id: None,
            email:       Some(email.into()),
            phone:       None
        }
    }
}
