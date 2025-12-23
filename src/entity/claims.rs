//! JWT Claims for authentication tokens.
//!
//! This module defines the [`Claims`] struct that represents the payload
//! of JWT tokens used for user authentication across the Revelation ecosystem.
//!
//! # Overview
//!
//! JWT (JSON Web Token) claims are the payload of authentication tokens.
//! They contain information about the authenticated user that can be
//! extracted from requests without database lookups.
//!
//! # Structure
//!
//! Claims follow the standard JWT claims with custom extensions:
//!
//! | Claim | Type | Description |
//! |-------|------|-------------|
//! | `sub` | `Uuid` | Subject - the user ID |
//! | `role` | `RUserRole` | User's role for authorization |
//! | `exp` | `usize` | Expiration time (Unix timestamp) |
//! | `iat` | `Option<usize>` | Issued at time (optional) |
//!
//! # Usage
//!
//! ## Creating Claims
//!
//! ```rust
//! use revelation_user::{Claims, RUserRole};
//! use uuid::Uuid;
//!
//! let claims = Claims::new(
//!     Uuid::now_v7(),
//!     RUserRole::User,
//!     1735689600 // expiration timestamp
//! );
//! ```
//!
//! ## Extracting from Requests
//!
//! With framework features enabled, claims can be extracted automatically:
//!
//! ```rust,ignore
//! // Axum
//! async fn handler(claims: Claims) -> impl IntoResponse {
//!     format!("User: {}", claims.user_id())
//! }
//!
//! // Actix-web
//! async fn handler(claims: Claims) -> impl Responder {
//!     format!("User: {}", claims.user_id())
//! }
//! ```
//!
//! # Security Considerations
//!
//! - Always validate `exp` claim before trusting the token
//! - Use [`is_expired()`](Claims::is_expired) to check expiration
//! - Store sensitive data in the database, not in claims

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Permissions, RUserRole, Role};

/// JWT claims for authentication tokens.
///
/// Represents the payload of a JWT token containing user identity
/// and authorization information.
///
/// # Standard Claims
///
/// - `sub` (subject): The user's unique identifier
/// - `exp` (expiration): Unix timestamp when the token expires
/// - `iat` (issued at): Unix timestamp when the token was issued (optional)
///
/// # Custom Claims
///
/// - `role`: The user's role for authorization decisions
///
/// # Examples
///
/// ```rust
/// use revelation_user::{Claims, RUserRole};
/// use uuid::Uuid;
///
/// // Create claims for a new token
/// let user_id = Uuid::now_v7();
/// let claims = Claims::new(user_id, RUserRole::User, 1735689600);
///
/// assert_eq!(claims.user_id(), user_id);
/// assert_eq!(claims.role, RUserRole::User);
/// ```
///
/// # Serialization
///
/// Claims serialize to standard JWT format:
///
/// ```rust
/// use revelation_user::{Claims, RUserRole};
/// use uuid::Uuid;
///
/// let claims = Claims::new(Uuid::nil(), RUserRole::Admin, 0);
/// let json = serde_json::to_string(&claims).unwrap();
///
/// assert!(json.contains("\"sub\""));
/// assert!(json.contains("\"role\""));
/// assert!(json.contains("\"exp\""));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject - the user's unique identifier.
    ///
    /// This corresponds to [`RUser::id`](crate::RUser::id).
    pub sub: Uuid,

    /// User's role for authorization.
    ///
    /// Used to make authorization decisions without database lookups.
    pub role: RUserRole,

    /// Expiration time as Unix timestamp (seconds since epoch).
    ///
    /// Tokens should not be accepted after this time.
    pub exp: usize,

    /// Issued at time as Unix timestamp (optional).
    ///
    /// Useful for token refresh logic and auditing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iat: Option<usize>,

    /// Custom permissions (optional).
    ///
    /// When set, these permissions override the role's default permissions.
    /// Useful for fine-grained access control beyond role-based permissions.
    ///
    /// If `None`, permissions are derived from the role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>
}

impl Claims {
    /// Create new claims for a JWT token.
    ///
    /// # Arguments
    ///
    /// * `sub` - The user's unique identifier (user ID)
    /// * `role` - The user's role for authorization
    /// * `exp` - Expiration time as Unix timestamp
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::{Duration, SystemTime, UNIX_EPOCH};
    ///
    /// use revelation_user::{Claims, RUserRole};
    /// use uuid::Uuid;
    ///
    /// let user_id = Uuid::now_v7();
    ///
    /// // Token expires in 1 hour
    /// let exp = SystemTime::now()
    ///     .duration_since(UNIX_EPOCH)
    ///     .unwrap()
    ///     .as_secs() as usize
    ///     + 3600;
    ///
    /// let claims = Claims::new(user_id, RUserRole::User, exp);
    /// ```
    #[must_use]
    pub fn new(sub: Uuid, role: RUserRole, exp: usize) -> Self {
        Self {
            sub,
            role,
            exp,
            iat: None,
            permissions: None
        }
    }

    /// Create claims with issued-at timestamp.
    ///
    /// # Arguments
    ///
    /// * `sub` - The user's unique identifier
    /// * `role` - The user's role
    /// * `exp` - Expiration time as Unix timestamp
    /// * `iat` - Issued at time as Unix timestamp
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{Claims, RUserRole};
    /// use uuid::Uuid;
    ///
    /// let now = 1700000000;
    /// let exp = now + 3600; // 1 hour later
    ///
    /// let claims = Claims::with_iat(Uuid::now_v7(), RUserRole::User, exp, now);
    ///
    /// assert_eq!(claims.iat, Some(now));
    /// ```
    #[must_use]
    pub fn with_iat(sub: Uuid, role: RUserRole, exp: usize, iat: usize) -> Self {
        Self {
            sub,
            role,
            exp,
            iat: Some(iat),
            permissions: None
        }
    }

    /// Create claims with custom permissions.
    ///
    /// Use this when you need fine-grained permissions that differ
    /// from the role's default permissions.
    ///
    /// # Arguments
    ///
    /// * `sub` - The user's unique identifier
    /// * `role` - The user's role
    /// * `exp` - Expiration time as Unix timestamp
    /// * `permissions` - Custom permissions to override role defaults
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{Claims, Permissions, RUserRole};
    /// use uuid::Uuid;
    ///
    /// // User with custom elevated permissions
    /// let claims = Claims::with_permissions(
    ///     Uuid::now_v7(),
    ///     RUserRole::User,
    ///     usize::MAX,
    ///     Permissions::READ | Permissions::WRITE | Permissions::EXPORT
    /// );
    ///
    /// assert!(claims.can(Permissions::EXPORT));
    /// ```
    #[must_use]
    pub fn with_permissions(
        sub: Uuid,
        role: RUserRole,
        exp: usize,
        permissions: Permissions
    ) -> Self {
        Self {
            sub,
            role,
            exp,
            iat: None,
            permissions: Some(permissions)
        }
    }

    /// Get the user ID from claims.
    ///
    /// This is a convenience method that returns the `sub` claim,
    /// which represents the user's unique identifier.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{Claims, RUserRole};
    /// use uuid::Uuid;
    ///
    /// let id = Uuid::now_v7();
    /// let claims = Claims::new(id, RUserRole::User, 0);
    ///
    /// assert_eq!(claims.user_id(), id);
    /// ```
    #[must_use]
    pub fn user_id(&self) -> Uuid {
        self.sub
    }

    /// Check if the claims have expired.
    ///
    /// Compares the `exp` claim against the current system time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{Claims, RUserRole};
    /// use uuid::Uuid;
    ///
    /// // Expired token (exp = 0 means Jan 1, 1970)
    /// let expired = Claims::new(Uuid::now_v7(), RUserRole::User, 0);
    /// assert!(expired.is_expired());
    ///
    /// // Future expiration
    /// let valid = Claims::new(Uuid::now_v7(), RUserRole::User, usize::MAX);
    /// assert!(!valid.is_expired());
    /// ```
    ///
    /// # Note
    ///
    /// This method uses system time, which could be manipulated.
    /// For high-security scenarios, use server-side time validation.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as usize)
            .unwrap_or(0);

        self.exp < now
    }

    /// Check if the user has admin role.
    ///
    /// Convenience method for authorization checks.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{Claims, RUserRole};
    /// use uuid::Uuid;
    ///
    /// let admin = Claims::new(Uuid::now_v7(), RUserRole::Admin, 0);
    /// let user = Claims::new(Uuid::now_v7(), RUserRole::User, 0);
    ///
    /// assert!(admin.is_admin());
    /// assert!(!user.is_admin());
    /// ```
    #[must_use]
    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    /// Check if the user has premium access.
    ///
    /// Returns `true` for both Premium and Admin roles.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{Claims, RUserRole};
    /// use uuid::Uuid;
    ///
    /// let premium = Claims::new(Uuid::now_v7(), RUserRole::Premium, 0);
    /// let admin = Claims::new(Uuid::now_v7(), RUserRole::Admin, 0);
    /// let user = Claims::new(Uuid::now_v7(), RUserRole::User, 0);
    ///
    /// assert!(premium.is_premium());
    /// assert!(admin.is_premium()); // Admins have premium access
    /// assert!(!user.is_premium());
    /// ```
    #[must_use]
    pub fn is_premium(&self) -> bool {
        self.role.is_premium()
    }

    /// Get the effective permissions for this claims.
    ///
    /// Returns custom permissions if set, otherwise derives
    /// permissions from the role.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{Claims, Permissions, RUserRole, Role};
    /// use uuid::Uuid;
    ///
    /// // Without custom permissions - uses role defaults
    /// let claims = Claims::new(Uuid::now_v7(), RUserRole::Admin, 0);
    /// assert_eq!(
    ///     claims.effective_permissions(),
    ///     RUserRole::Admin.permissions()
    /// );
    ///
    /// // With custom permissions - uses those instead
    /// let claims = Claims::with_permissions(
    ///     Uuid::now_v7(),
    ///     RUserRole::User,
    ///     0,
    ///     Permissions::READ | Permissions::EXPORT
    /// );
    /// assert!(claims.effective_permissions().contains(Permissions::EXPORT));
    /// ```
    #[must_use]
    pub fn effective_permissions(&self) -> Permissions {
        self.permissions.unwrap_or_else(|| self.role.permissions())
    }

    /// Check if the claims have the specified permission.
    ///
    /// Uses custom permissions if set, otherwise checks role permissions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{Claims, Permissions, RUserRole};
    /// use uuid::Uuid;
    ///
    /// let admin = Claims::new(Uuid::now_v7(), RUserRole::Admin, 0);
    /// assert!(admin.can(Permissions::DELETE));
    ///
    /// let user = Claims::new(Uuid::now_v7(), RUserRole::User, 0);
    /// assert!(!user.can(Permissions::DELETE));
    /// ```
    #[must_use]
    pub fn can(&self, permission: Permissions) -> bool {
        self.effective_permissions().contains(permission)
    }

    /// Check if the claims have all the specified permissions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{Claims, Permissions, RUserRole};
    /// use uuid::Uuid;
    ///
    /// let admin = Claims::new(Uuid::now_v7(), RUserRole::Admin, 0);
    /// let required = Permissions::READ | Permissions::WRITE | Permissions::DELETE;
    /// assert!(admin.can_all(required));
    ///
    /// let user = Claims::new(Uuid::now_v7(), RUserRole::User, 0);
    /// assert!(!user.can_all(required));
    /// ```
    #[must_use]
    pub fn can_all(&self, permissions: Permissions) -> bool {
        self.effective_permissions().contains(permissions)
    }

    /// Check if the claims have any of the specified permissions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{Claims, Permissions, RUserRole};
    /// use uuid::Uuid;
    ///
    /// let user = Claims::new(Uuid::now_v7(), RUserRole::User, 0);
    /// let any_of = Permissions::ADMIN | Permissions::READ;
    /// assert!(user.can_any(any_of)); // Has READ
    /// ```
    #[must_use]
    pub fn can_any(&self, permissions: Permissions) -> bool {
        self.effective_permissions().intersects(permissions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_claims_without_iat() {
        let claims = Claims::new(Uuid::nil(), RUserRole::User, 1000);
        assert!(claims.iat.is_none());
    }

    #[test]
    fn with_iat_sets_iat() {
        let claims = Claims::with_iat(Uuid::nil(), RUserRole::User, 1000, 500);
        assert_eq!(claims.iat, Some(500));
    }

    #[test]
    fn user_id_returns_sub() {
        let id = Uuid::now_v7();
        let claims = Claims::new(id, RUserRole::User, 0);
        assert_eq!(claims.user_id(), id);
    }

    #[test]
    fn is_expired_returns_true_for_past() {
        let claims = Claims::new(Uuid::nil(), RUserRole::User, 0);
        assert!(claims.is_expired());
    }

    #[test]
    fn is_expired_returns_false_for_future() {
        let claims = Claims::new(Uuid::nil(), RUserRole::User, usize::MAX);
        assert!(!claims.is_expired());
    }

    #[test]
    fn is_admin_checks_role() {
        let admin = Claims::new(Uuid::nil(), RUserRole::Admin, 0);
        let user = Claims::new(Uuid::nil(), RUserRole::User, 0);

        assert!(admin.is_admin());
        assert!(!user.is_admin());
    }

    #[test]
    fn serializes_without_iat_when_none() {
        let claims = Claims::new(Uuid::nil(), RUserRole::User, 0);
        let json = serde_json::to_string(&claims).unwrap();
        assert!(!json.contains("iat"));
    }

    #[test]
    fn is_premium_checks_role() {
        let premium = Claims::new(Uuid::nil(), RUserRole::Premium, 0);
        let admin = Claims::new(Uuid::nil(), RUserRole::Admin, 0);
        let user = Claims::new(Uuid::nil(), RUserRole::User, 0);

        assert!(premium.is_premium());
        assert!(admin.is_premium());
        assert!(!user.is_premium());
    }

    #[test]
    fn new_creates_claims_without_permissions() {
        let claims = Claims::new(Uuid::nil(), RUserRole::User, 0);
        assert!(claims.permissions.is_none());
    }

    #[test]
    fn with_permissions_sets_permissions() {
        let perms = Permissions::READ | Permissions::WRITE;
        let claims = Claims::with_permissions(Uuid::nil(), RUserRole::User, 0, perms);
        assert_eq!(claims.permissions, Some(perms));
    }

    #[test]
    fn effective_permissions_uses_role_when_no_custom() {
        let claims = Claims::new(Uuid::nil(), RUserRole::Admin, 0);
        assert_eq!(claims.effective_permissions(), Permissions::all());
    }

    #[test]
    fn effective_permissions_uses_custom_when_set() {
        let custom = Permissions::READ | Permissions::EXPORT;
        let claims = Claims::with_permissions(Uuid::nil(), RUserRole::Admin, 0, custom);
        assert_eq!(claims.effective_permissions(), custom);
    }

    #[test]
    fn can_checks_permission() {
        let admin = Claims::new(Uuid::nil(), RUserRole::Admin, 0);
        assert!(admin.can(Permissions::DELETE));

        let user = Claims::new(Uuid::nil(), RUserRole::User, 0);
        assert!(!user.can(Permissions::DELETE));
    }

    #[test]
    fn can_all_checks_multiple_permissions() {
        let required = Permissions::READ | Permissions::WRITE;

        let admin = Claims::new(Uuid::nil(), RUserRole::Admin, 0);
        assert!(admin.can_all(required));

        let user = Claims::new(Uuid::nil(), RUserRole::User, 0);
        assert!(!user.can_all(required));
    }

    #[test]
    fn can_any_checks_any_permission() {
        let any_of = Permissions::ADMIN | Permissions::READ;

        let user = Claims::new(Uuid::nil(), RUserRole::User, 0);
        assert!(user.can_any(any_of));

        let no_match = Permissions::ADMIN | Permissions::DELETE;
        assert!(!user.can_any(no_match));
    }

    #[test]
    fn serializes_without_permissions_when_none() {
        let claims = Claims::new(Uuid::nil(), RUserRole::User, 0);
        let json = serde_json::to_string(&claims).unwrap();
        assert!(!json.contains("permissions"));
    }

    #[test]
    fn serializes_with_permissions_when_set() {
        let perms = Permissions::READ | Permissions::WRITE;
        let claims = Claims::with_permissions(Uuid::nil(), RUserRole::User, 0, perms);
        let json = serde_json::to_string(&claims).unwrap();
        assert!(json.contains("permissions"));
    }
}
