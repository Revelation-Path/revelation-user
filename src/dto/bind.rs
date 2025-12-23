//! Contact binding DTOs.

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Bind Telegram account to user.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct BindTelegram {
    /// Telegram user ID.
    #[validate(range(min = 1))]
    pub telegram_id: i64
}

/// Bind email address to user.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct BindEmail {
    /// Email address.
    #[validate(email)]
    pub email: String
}

/// Bind phone number to user.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct BindPhone {
    /// Phone in E.164 format (+XXXXXXXXXXX).
    #[validate(regex(path = *crate::PHONE_REGEX))]
    pub phone: String
}
