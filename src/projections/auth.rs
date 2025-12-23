//! Authentication user projection - for JWT/session data.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{RUser, RUserRole};

/// User data for authentication context.
///
/// Contains fields needed for authorization decisions.
///
/// # Examples
///
/// ```
/// use revelation_user::{RUser, RUserAuth, RUserRole};
///
/// let user = RUser::from_telegram(123456);
/// let auth = RUserAuth::from_user(&user, RUserRole::User);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct RUserAuth {
    /// User ID.
    pub id: Uuid,

    /// Telegram user ID (if authenticated via Telegram).
    pub telegram_id: Option<i64>,

    /// User role.
    pub role: RUserRole
}

impl RUserAuth {
    /// Create from user with specified role.
    #[must_use]
    pub fn from_user(user: &RUser, role: RUserRole) -> Self {
        Self {
            id: user.id,
            telegram_id: user.telegram_id,
            role
        }
    }

    /// Create with default User role.
    #[must_use]
    pub fn from_user_default(user: &RUser) -> Self {
        Self::from_user(user, RUserRole::User)
    }
}
