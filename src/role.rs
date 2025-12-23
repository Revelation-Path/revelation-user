//! User role enumeration for authorization.
//!
//! This module provides the [`RUserRole`] enum for role-based
//! access control (RBAC) in the Revelation ecosystem.
//!
//! # Role Hierarchy
//!
//! Roles have an implicit hierarchy:
//!
//! ```text
//! Admin > Premium > User
//! ```
//!
//! - **Admin**: Full access to all features
//! - **Premium**: Access to premium features + user features
//! - **User**: Basic access only
//!
//! # Database Integration
//!
//! With the `db` feature, [`RUserRole`] maps to PostgreSQL enum:
//!
//! ```sql
//! CREATE TYPE user_role AS ENUM ('user', 'premium', 'admin');
//! ```
//!
//! # Examples
//!
//! ```rust
//! use revelation_user::RUserRole;
//!
//! let role = RUserRole::Premium;
//!
//! // Check capabilities
//! assert!(role.is_premium()); // Has premium access
//! assert!(!role.is_admin()); // Not an admin
//!
//! // Default is User
//! assert_eq!(RUserRole::default(), RUserRole::User);
//! ```

use serde::{Deserialize, Serialize};

/// User role for authorization decisions.
///
/// Defines the access level and capabilities of a user.
///
/// # Hierarchy
///
/// | Role | Premium Access | Admin Access |
/// |------|----------------|--------------|
/// | `User` | No | No |
/// | `Premium` | Yes | No |
/// | `Admin` | Yes | Yes |
///
/// # Default
///
/// The default role is [`RUserRole::User`].
///
/// # Examples
///
/// ```rust
/// use revelation_user::RUserRole;
///
/// // Default role
/// let role = RUserRole::default();
/// assert_eq!(role, RUserRole::User);
///
/// // Check access levels
/// assert!(!RUserRole::User.is_premium());
/// assert!(RUserRole::Premium.is_premium());
/// assert!(RUserRole::Admin.is_premium()); // Admins have premium
/// assert!(RUserRole::Admin.is_admin());
/// ```
///
/// # Serialization
///
/// Roles serialize to lowercase snake_case:
///
/// ```rust
/// use revelation_user::RUserRole;
///
/// assert_eq!(serde_json::to_string(&RUserRole::User).unwrap(), "\"user\"");
/// assert_eq!(
///     serde_json::to_string(&RUserRole::Premium).unwrap(),
///     "\"premium\""
/// );
/// assert_eq!(
///     serde_json::to_string(&RUserRole::Admin).unwrap(),
///     "\"admin\""
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(
    feature = "db",
    sqlx(type_name = "user_role", rename_all = "snake_case")
)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub enum RUserRole {
    /// Regular user with basic access.
    ///
    /// This is the default role for new users.
    #[default]
    User,

    /// Premium user with access to premium features.
    ///
    /// Includes all User capabilities plus premium content.
    Premium,

    /// Administrator with full access.
    ///
    /// Has all Premium capabilities plus admin functions.
    Admin
}

impl RUserRole {
    /// Check if this role has admin privileges.
    ///
    /// Only [`RUserRole::Admin`] returns `true`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUserRole;
    ///
    /// assert!(RUserRole::Admin.is_admin());
    /// assert!(!RUserRole::Premium.is_admin());
    /// assert!(!RUserRole::User.is_admin());
    /// ```
    #[must_use]
    pub const fn is_admin(&self) -> bool {
        matches!(self, Self::Admin)
    }

    /// Check if this role has premium access.
    ///
    /// Both [`RUserRole::Premium`] and [`RUserRole::Admin`] return `true`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUserRole;
    ///
    /// assert!(RUserRole::Admin.is_premium());
    /// assert!(RUserRole::Premium.is_premium());
    /// assert!(!RUserRole::User.is_premium());
    /// ```
    #[must_use]
    pub const fn is_premium(&self) -> bool {
        matches!(self, Self::Premium | Self::Admin)
    }

    /// Check if this is a regular user role.
    ///
    /// Only [`RUserRole::User`] returns `true`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUserRole;
    ///
    /// assert!(RUserRole::User.is_user());
    /// assert!(!RUserRole::Premium.is_user());
    /// assert!(!RUserRole::Admin.is_user());
    /// ```
    #[must_use]
    pub const fn is_user(&self) -> bool {
        matches!(self, Self::User)
    }

    /// Returns the role as a lowercase string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::RUserRole;
    ///
    /// assert_eq!(RUserRole::User.as_str(), "user");
    /// assert_eq!(RUserRole::Premium.as_str(), "premium");
    /// assert_eq!(RUserRole::Admin.as_str(), "admin");
    /// ```
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Premium => "premium",
            Self::Admin => "admin"
        }
    }
}

impl core::fmt::Display for RUserRole {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_user() {
        assert_eq!(RUserRole::default(), RUserRole::User);
    }

    #[test]
    fn is_admin_only_for_admin() {
        assert!(RUserRole::Admin.is_admin());
        assert!(!RUserRole::Premium.is_admin());
        assert!(!RUserRole::User.is_admin());
    }

    #[test]
    fn is_premium_for_premium_and_admin() {
        assert!(RUserRole::Admin.is_premium());
        assert!(RUserRole::Premium.is_premium());
        assert!(!RUserRole::User.is_premium());
    }

    #[test]
    fn serializes_correctly() {
        assert_eq!(serde_json::to_string(&RUserRole::User).unwrap(), "\"user\"");
        assert_eq!(
            serde_json::to_string(&RUserRole::Premium).unwrap(),
            "\"premium\""
        );
        assert_eq!(
            serde_json::to_string(&RUserRole::Admin).unwrap(),
            "\"admin\""
        );
    }

    #[test]
    fn deserializes_correctly() {
        assert_eq!(
            serde_json::from_str::<RUserRole>("\"user\"").unwrap(),
            RUserRole::User
        );
        assert_eq!(
            serde_json::from_str::<RUserRole>("\"admin\"").unwrap(),
            RUserRole::Admin
        );
    }

    #[test]
    fn display_impl() {
        assert_eq!(format!("{}", RUserRole::User), "user");
        assert_eq!(format!("{}", RUserRole::Admin), "admin");
    }
}
