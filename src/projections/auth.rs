//! Authentication user projection for JWT/session context.
//!
//! This module provides [`RUserAuth`], a projection optimized for
//! authentication and authorization contexts. It contains the
//! minimal set of fields needed for access control decisions.
//!
//! # Use Cases
//!
//! - **JWT Payload**: Embed in JWT tokens for stateless auth
//! - **Session Storage**: Store in server-side sessions
//! - **Authorization Checks**: Determine user permissions
//!
//! # Design
//!
//! Unlike [`RUserPublic`], this projection includes:
//! - User role for permission checks
//! - Telegram ID for authentication verification
//!
//! But excludes personal data like name, email, phone.
//!
//! # Examples
//!
//! ## Creating from User
//!
//! ```rust
//! use revelation_user::{RUser, RUserAuth, RUserRole};
//!
//! let user = RUser::from_telegram(123456789);
//!
//! // With explicit role
//! let auth = RUserAuth::from_user(&user, RUserRole::Premium);
//! assert!(auth.role.is_premium());
//!
//! // With default User role
//! let auth = RUserAuth::from_user_default(&user);
//! assert!(auth.role.is_user());
//! ```
//!
//! ## Authorization Checks
//!
//! ```rust
//! use revelation_user::{RUserAuth, RUserRole};
//! use uuid::Uuid;
//!
//! fn check_admin_access(auth: &RUserAuth) -> bool {
//!     auth.role.is_admin()
//! }
//!
//! fn check_premium_content(auth: &RUserAuth) -> bool {
//!     auth.role.is_premium() // Includes both Premium and Admin
//! }
//! ```
//!
//! [`RUserPublic`]: crate::RUserPublic

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{RUser, RUserRole};

/// User data for authentication and authorization context.
///
/// This projection contains the minimal set of fields needed
/// for access control decisions in JWT tokens or sessions.
///
/// # Fields
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `id` | `Uuid` | Unique user identifier |
/// | `telegram_id` | `Option<i64>` | Telegram ID (if authenticated via Telegram) |
/// | `role` | `RUserRole` | User's authorization role |
///
/// # Role-Based Access Control
///
/// The `role` field supports hierarchical permissions:
///
/// ```rust
/// use revelation_user::{RUserAuth, RUserRole};
/// use uuid::Uuid;
///
/// let admin_auth = RUserAuth {
///     id:          Uuid::now_v7(),
///     telegram_id: Some(123),
///     role:        RUserRole::Admin
/// };
///
/// // Admins have all permissions
/// assert!(admin_auth.role.is_admin());
/// assert!(admin_auth.role.is_premium());
///
/// let user_auth = RUserAuth {
///     id:          Uuid::now_v7(),
///     telegram_id: None,
///     role:        RUserRole::User
/// };
///
/// // Regular users have basic permissions only
/// assert!(!user_auth.role.is_admin());
/// assert!(!user_auth.role.is_premium());
/// ```
///
/// # Examples
///
/// ## From User with Role
///
/// ```rust
/// use revelation_user::{RUser, RUserAuth, RUserRole};
///
/// let user = RUser::from_telegram(123456789);
/// let auth = RUserAuth::from_user(&user, RUserRole::Premium);
///
/// assert_eq!(auth.telegram_id, Some(123456789));
/// assert!(auth.role.is_premium());
/// ```
///
/// ## JWT Token Payload
///
/// ```rust
/// use revelation_user::{RUser, RUserAuth, RUserRole};
///
/// let user = RUser::from_email("admin@example.com");
/// let auth = RUserAuth::from_user(&user, RUserRole::Admin);
///
/// // Serialize for JWT
/// let json = serde_json::to_string(&auth).unwrap();
/// assert!(json.contains("admin"));
///
/// // Deserialize from JWT
/// let decoded: RUserAuth = serde_json::from_str(&json).unwrap();
/// assert!(decoded.role.is_admin());
/// ```
///
/// ## Equality Comparison
///
/// ```rust
/// use revelation_user::{RUserAuth, RUserRole};
/// use uuid::Uuid;
///
/// let id = Uuid::now_v7();
///
/// let auth1 = RUserAuth {
///     id,
///     telegram_id: Some(123),
///     role: RUserRole::User
/// };
///
/// let auth2 = RUserAuth {
///     id,
///     telegram_id: Some(123),
///     role: RUserRole::User
/// };
///
/// assert_eq!(auth1, auth2);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct RUserAuth {
    /// Unique user identifier.
    ///
    /// This is the same UUID from the source [`RUser`].
    ///
    /// [`RUser`]: crate::RUser
    pub id: Uuid,

    /// Telegram user ID if authenticated via Telegram.
    ///
    /// Useful for verifying Telegram-based authentication
    /// or linking with Telegram bot interactions.
    pub telegram_id: Option<i64>,

    /// User's authorization role.
    ///
    /// Determines access permissions. Use helper methods
    /// for hierarchical checks:
    /// - [`is_admin()`](RUserRole::is_admin) - Admin only
    /// - [`is_premium()`](RUserRole::is_premium) - Premium or Admin
    /// - [`is_user()`](RUserRole::is_user) - Regular user only
    pub role: RUserRole
}

impl RUserAuth {
    /// Create authentication projection from user with specified role.
    ///
    /// # Arguments
    ///
    /// * `user` - Reference to the source [`RUser`]
    /// * `role` - Authorization role to assign
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{RUser, RUserAuth, RUserRole};
    ///
    /// let user = RUser::from_telegram(123456789);
    /// let auth = RUserAuth::from_user(&user, RUserRole::Admin);
    ///
    /// assert_eq!(auth.id, user.id);
    /// assert!(auth.role.is_admin());
    /// ```
    ///
    /// [`RUser`]: crate::RUser
    #[must_use]
    pub fn from_user(user: &RUser, role: RUserRole) -> Self {
        Self {
            id: user.id,
            telegram_id: user.telegram_id,
            role
        }
    }

    /// Create authentication projection with default [`RUserRole::User`] role.
    ///
    /// Convenience method equivalent to `from_user(user, RUserRole::User)`.
    ///
    /// # Arguments
    ///
    /// * `user` - Reference to the source [`RUser`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{RUser, RUserAuth, RUserRole};
    ///
    /// let user = RUser::from_email("user@example.com");
    /// let auth = RUserAuth::from_user_default(&user);
    ///
    /// assert_eq!(auth.role, RUserRole::User);
    /// assert!(!auth.role.is_premium());
    /// ```
    ///
    /// [`RUser`]: crate::RUser
    /// [`RUserRole::User`]: crate::RUserRole::User
    #[must_use]
    pub fn from_user_default(user: &RUser) -> Self {
        Self::from_user(user, RUserRole::User)
    }

    /// Check if this user has admin privileges.
    ///
    /// Convenience method that delegates to [`RUserRole::is_admin`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{RUser, RUserAuth, RUserRole};
    ///
    /// let user = RUser::from_telegram(123);
    /// let auth = RUserAuth::from_user(&user, RUserRole::Admin);
    ///
    /// assert!(auth.is_admin());
    /// ```
    #[must_use]
    pub const fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    /// Check if this user has premium access.
    ///
    /// Returns `true` for both [`RUserRole::Premium`] and [`RUserRole::Admin`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{RUser, RUserAuth, RUserRole};
    ///
    /// let user = RUser::from_telegram(123);
    ///
    /// let premium = RUserAuth::from_user(&user, RUserRole::Premium);
    /// assert!(premium.is_premium());
    ///
    /// let admin = RUserAuth::from_user(&user, RUserRole::Admin);
    /// assert!(admin.is_premium()); // Admins have premium
    ///
    /// let regular = RUserAuth::from_user(&user, RUserRole::User);
    /// assert!(!regular.is_premium());
    /// ```
    ///
    /// [`RUserRole::Premium`]: crate::RUserRole::Premium
    /// [`RUserRole::Admin`]: crate::RUserRole::Admin
    #[must_use]
    pub const fn is_premium(&self) -> bool {
        self.role.is_premium()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_user_copies_fields() {
        let user = RUser::from_telegram(123456789);
        let auth = RUserAuth::from_user(&user, RUserRole::Premium);

        assert_eq!(auth.id, user.id);
        assert_eq!(auth.telegram_id, Some(123456789));
        assert_eq!(auth.role, RUserRole::Premium);
    }

    #[test]
    fn from_user_default_uses_user_role() {
        let user = RUser::from_email("test@example.com");
        let auth = RUserAuth::from_user_default(&user);

        assert_eq!(auth.role, RUserRole::User);
    }

    #[test]
    fn is_admin_works() {
        let user = RUser::from_telegram(123);

        let admin = RUserAuth::from_user(&user, RUserRole::Admin);
        assert!(admin.is_admin());

        let regular = RUserAuth::from_user(&user, RUserRole::User);
        assert!(!regular.is_admin());
    }

    #[test]
    fn is_premium_includes_admin() {
        let user = RUser::from_telegram(123);

        let admin = RUserAuth::from_user(&user, RUserRole::Admin);
        assert!(admin.is_premium());

        let premium = RUserAuth::from_user(&user, RUserRole::Premium);
        assert!(premium.is_premium());

        let regular = RUserAuth::from_user(&user, RUserRole::User);
        assert!(!regular.is_premium());
    }

    #[test]
    fn equality_works() {
        let id = Uuid::now_v7();

        let auth1 = RUserAuth {
            id,
            telegram_id: Some(123),
            role: RUserRole::User
        };

        let auth2 = RUserAuth {
            id,
            telegram_id: Some(123),
            role: RUserRole::User
        };

        assert_eq!(auth1, auth2);
    }

    #[test]
    fn serialization_roundtrip() {
        let auth = RUserAuth {
            id:          Uuid::nil(),
            telegram_id: Some(123),
            role:        RUserRole::Admin
        };

        let json = serde_json::to_string(&auth).unwrap();
        let decoded: RUserAuth = serde_json::from_str(&json).unwrap();

        assert_eq!(auth, decoded);
    }
}
