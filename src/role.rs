//! User role enumeration.

use serde::{Deserialize, Serialize};

/// User role for authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(
    feature = "db",
    sqlx(type_name = "user_role", rename_all = "snake_case")
)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub enum RUserRole {
    /// Regular user.
    #[default]
    User,
    /// Premium user.
    Premium,
    /// Administrator.
    Admin
}

impl RUserRole {
    /// Check if role has admin privileges.
    #[must_use]
    pub fn is_admin(&self) -> bool {
        matches!(self, Self::Admin)
    }

    /// Check if role has premium access.
    #[must_use]
    pub fn is_premium(&self) -> bool {
        matches!(self, Self::Premium | Self::Admin)
    }
}
