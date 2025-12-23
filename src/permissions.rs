//! Permission-based Role-Based Access Control (RBAC).
//!
//! This module provides a professional, production-ready permission system
//! inspired by AWS IAM, Kubernetes RBAC, and Google Cloud IAM.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                        User                                     │
//! │                          │                                      │
//! │                          ▼                                      │
//! │                    ┌──────────┐                                 │
//! │                    │   Role   │ ◄─── RUserRole, Custom roles    │
//! │                    └────┬─────┘                                 │
//! │                         │                                       │
//! │                         ▼                                       │
//! │              ┌─────────────────────┐                           │
//! │              │    Permissions      │ ◄─── Bitflags (fast)      │
//! │              │  READ | WRITE | ... │                           │
//! │              └─────────────────────┘                           │
//! │                         │                                       │
//! │                         ▼                                       │
//! │              ┌─────────────────────┐                           │
//! │              │   Access Decision   │                           │
//! │              │   ALLOW / DENY      │                           │
//! │              └─────────────────────┘                           │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Quick Start
//!
//! ```rust
//! use revelation_user::{Permissions, RUserRole, Role};
//!
//! // Check permissions on a role
//! let admin = RUserRole::Admin;
//! assert!(admin.can(Permissions::DELETE));
//! assert!(admin.can(Permissions::all()));
//!
//! // Combine permissions
//! let editor_perms = Permissions::READ | Permissions::WRITE;
//! assert!(editor_perms.contains(Permissions::READ));
//!
//! // Check multiple permissions at once
//! let required = Permissions::READ | Permissions::WRITE;
//! assert!(admin.permissions().contains(required));
//! ```
//!
//! # Custom Roles
//!
//! Implement the [`Role`] trait for custom role types:
//!
//! ```rust
//! use revelation_user::{Permissions, Role};
//!
//! #[derive(Debug, Clone, Copy)]
//! enum CustomRole {
//!     Viewer,
//!     Editor,
//!     Moderator,
//!     SuperAdmin
//! }
//!
//! impl Role for CustomRole {
//!     fn permissions(&self) -> Permissions {
//!         match self {
//!             Self::Viewer => Permissions::READ,
//!             Self::Editor => Permissions::READ | Permissions::WRITE,
//!             Self::Moderator => Permissions::READ | Permissions::WRITE | Permissions::DELETE,
//!             Self::SuperAdmin => Permissions::all()
//!         }
//!     }
//!
//!     fn name(&self) -> &'static str {
//!         match self {
//!             Self::Viewer => "viewer",
//!             Self::Editor => "editor",
//!             Self::Moderator => "moderator",
//!             Self::SuperAdmin => "super_admin"
//!         }
//!     }
//! }
//!
//! let mod_role = CustomRole::Moderator;
//! assert!(mod_role.can(Permissions::DELETE));
//! assert!(!mod_role.can(Permissions::ADMIN));
//! ```
//!
//! # Permission Hierarchy
//!
//! | Permission | Bit | Description |
//! |------------|-----|-------------|
//! | `READ` | 0x0001 | View resources |
//! | `WRITE` | 0x0002 | Create and modify resources |
//! | `DELETE` | 0x0004 | Remove resources |
//! | `ADMIN` | 0x0008 | Administrative operations |
//! | `MANAGE_USERS` | 0x0010 | User management |
//! | `MANAGE_ROLES` | 0x0020 | Role assignment |
//! | `BILLING` | 0x0040 | Billing and payments |
//! | `AUDIT` | 0x0080 | View audit logs |
//! | `EXPORT` | 0x0100 | Export data |
//! | `IMPORT` | 0x0200 | Import data |
//! | `API_ACCESS` | 0x0400 | API access |
//! | `PREMIUM` | 0x0800 | Premium features |
//!
//! # Preset Permission Sets
//!
//! | Preset | Permissions |
//! |--------|-------------|
//! | `VIEWER` | READ |
//! | `EDITOR` | READ, WRITE |
//! | `MANAGER` | READ, WRITE, DELETE, MANAGE_USERS |
//!
//! Use `Permissions::all()` for full access.
//!
//! # Serialization
//!
//! Permissions serialize to a numeric value for efficient storage:
//!
//! ```rust
//! use revelation_user::Permissions;
//!
//! let perms = Permissions::READ | Permissions::WRITE;
//! let json = serde_json::to_string(&perms).unwrap();
//! assert_eq!(json, "3"); // 0b0011
//!
//! let restored: Permissions = serde_json::from_str(&json).unwrap();
//! assert_eq!(perms, restored);
//! ```

bitflags::bitflags! {
    /// Bitflag-based permissions for fine-grained access control.
    ///
    /// Permissions can be combined using bitwise operators (`|`, `&`, `^`)
    /// and checked efficiently using the `contains` method.
    ///
    /// # Performance
    ///
    /// Permission checks are O(1) bitwise operations, making them
    /// extremely fast for hot paths in request handlers.
    ///
    /// # Serialization
    ///
    /// - **Serialize**: Always outputs a number (compact for JWT/DB)
    /// - **Deserialize**: Accepts both number and string formats
    /// - **Display**: Human-readable format for logs
    ///
    /// ```rust
    /// use revelation_user::Permissions;
    ///
    /// let perms = Permissions::READ | Permissions::WRITE;
    ///
    /// // Serializes to number
    /// let json = serde_json::to_string(&perms).unwrap();
    /// assert_eq!(json, "3");
    ///
    /// // Display is human-readable
    /// assert_eq!(format!("{}", perms), "read, write");
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::Permissions;
    ///
    /// // Single permission
    /// let read = Permissions::READ;
    ///
    /// // Combined permissions
    /// let editor = Permissions::READ | Permissions::WRITE;
    ///
    /// // Check if permission is present
    /// assert!(editor.contains(Permissions::READ));
    /// assert!(!editor.contains(Permissions::DELETE));
    ///
    /// // Check multiple permissions
    /// let required = Permissions::READ | Permissions::WRITE;
    /// assert!(editor.contains(required));
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Permissions: u32 {
        // ═══════════════════════════════════════════════════════════════
        // Core CRUD Operations (0x000F)
        // ═══════════════════════════════════════════════════════════════

        /// Read/view resources.
        ///
        /// Basic permission for viewing content, user profiles,
        /// and other read-only operations.
        const READ = 0x0001;

        /// Create and modify resources.
        ///
        /// Allows creating new content and editing existing content
        /// owned by the user.
        const WRITE = 0x0002;

        /// Delete resources.
        ///
        /// Allows removing content. Usually combined with WRITE.
        const DELETE = 0x0004;

        /// Administrative operations.
        ///
        /// System-level operations, configuration changes,
        /// and other administrative tasks.
        const ADMIN = 0x0008;

        // ═══════════════════════════════════════════════════════════════
        // Management Operations (0x00F0)
        // ═══════════════════════════════════════════════════════════════

        /// Manage users.
        ///
        /// Create, modify, and delete user accounts.
        /// View user details and activity.
        const MANAGE_USERS = 0x0010;

        /// Manage roles and permissions.
        ///
        /// Assign and revoke roles, modify permission sets.
        const MANAGE_ROLES = 0x0020;

        /// Billing and payment operations.
        ///
        /// View invoices, manage subscriptions, update payment methods.
        const BILLING = 0x0040;

        /// View audit logs.
        ///
        /// Access to security logs, activity history, and audit trails.
        const AUDIT = 0x0080;

        // ═══════════════════════════════════════════════════════════════
        // Data Operations (0x0F00)
        // ═══════════════════════════════════════════════════════════════

        /// Export data.
        ///
        /// Download data in various formats (CSV, JSON, etc.).
        const EXPORT = 0x0100;

        /// Import data.
        ///
        /// Upload and process bulk data imports.
        const IMPORT = 0x0200;

        /// API access.
        ///
        /// Access to REST/GraphQL APIs for programmatic access.
        const API_ACCESS = 0x0400;

        /// Premium features.
        ///
        /// Access to premium/paid features.
        const PREMIUM = 0x0800;

        // ═══════════════════════════════════════════════════════════════
        // Presets
        // ═══════════════════════════════════════════════════════════════

        /// Viewer preset: read-only access.
        const VIEWER = Self::READ.bits();

        /// Editor preset: read and write access.
        const EDITOR = Self::READ.bits() | Self::WRITE.bits();

        /// Manager preset: full content management.
        const MANAGER = Self::READ.bits()
            | Self::WRITE.bits()
            | Self::DELETE.bits()
            | Self::MANAGE_USERS.bits();
    }
}

// Custom serialization: always serialize as number (compact for JWT)
impl serde::Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_u32(self.bits())
    }
}

// Custom deserialization: accept both number and string
impl<'de> serde::Deserialize<'de> for Permissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        struct PermissionsVisitor;

        impl serde::de::Visitor<'_> for PermissionsVisitor {
            type Value = Permissions;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a number or permission string")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error
            {
                Permissions::from_bits(value as u32)
                    .ok_or_else(|| E::custom(format!("invalid permission bits: {value}")))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error
            {
                if value < 0 {
                    return Err(E::custom("permissions cannot be negative"));
                }
                self.visit_u64(value as u64)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error
            {
                parse_permissions(value).map_err(E::custom)
            }
        }

        deserializer.deserialize_any(PermissionsVisitor)
    }
}

/// Parse permissions from a string like "read, write" or "READ | WRITE".
fn parse_permissions(s: &str) -> Result<Permissions, String> {
    let mut result = Permissions::empty();

    for part in s.split([',', '|']) {
        let name = part.trim().to_lowercase();
        let perm = match name.as_str() {
            "read" => Permissions::READ,
            "write" => Permissions::WRITE,
            "delete" => Permissions::DELETE,
            "admin" => Permissions::ADMIN,
            "manage_users" => Permissions::MANAGE_USERS,
            "manage_roles" => Permissions::MANAGE_ROLES,
            "billing" => Permissions::BILLING,
            "audit" => Permissions::AUDIT,
            "export" => Permissions::EXPORT,
            "import" => Permissions::IMPORT,
            "api_access" => Permissions::API_ACCESS,
            "premium" => Permissions::PREMIUM,
            "" => continue,
            _ => return Err(format!("unknown permission: {name}"))
        };
        result |= perm;
    }

    Ok(result)
}

impl Permissions {
    /// Check if these permissions satisfy the required permissions.
    ///
    /// Returns `true` if all bits in `required` are set in `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::Permissions;
    ///
    /// let user_perms = Permissions::READ | Permissions::WRITE;
    /// let required = Permissions::READ;
    ///
    /// assert!(user_perms.satisfies(required));
    /// assert!(!user_perms.satisfies(Permissions::ADMIN));
    /// ```
    #[inline]
    #[must_use]
    pub const fn satisfies(self, required: Self) -> bool {
        self.contains(required)
    }

    /// Create permissions from a raw bits value.
    ///
    /// Returns `None` if the bits contain invalid flags.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::Permissions;
    ///
    /// let perms = Permissions::from_bits_checked(0x0003);
    /// assert_eq!(perms, Some(Permissions::READ | Permissions::WRITE));
    ///
    /// // Invalid bits return None
    /// let invalid = Permissions::from_bits_checked(0xFFFF_FFFF);
    /// assert!(invalid.is_none());
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_bits_checked(bits: u32) -> Option<Self> {
        Self::from_bits(bits)
    }

    /// Check if no permissions are set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::Permissions;
    ///
    /// assert!(Permissions::empty().is_none());
    /// assert!(!Permissions::READ.is_none());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_none(self) -> bool {
        self.is_empty()
    }

    /// Get the raw bits value.
    ///
    /// Useful for database storage or serialization.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::Permissions;
    ///
    /// let perms = Permissions::READ | Permissions::WRITE;
    /// assert_eq!(perms.as_u32(), 0x0003);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.bits()
    }

    /// Create permissions from raw bits, truncating invalid bits.
    ///
    /// Unlike `from_bits`, this never fails - it simply ignores
    /// any bits that don't correspond to valid permissions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::Permissions;
    ///
    /// // Invalid bits are ignored
    /// let perms = Permissions::from_bits_truncating(0xFFFF_FFFF);
    /// assert_eq!(perms, Permissions::all());
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_bits_truncating(bits: u32) -> Self {
        Self::from_bits_truncate(bits)
    }
}

impl Default for Permissions {
    /// Default permissions: READ only.
    ///
    /// New users get read access by default.
    #[inline]
    fn default() -> Self {
        Self::READ
    }
}

impl core::fmt::Display for Permissions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_empty() {
            return write!(f, "none");
        }

        let mut parts = Vec::new();

        if self.contains(Self::READ) {
            parts.push("read");
        }
        if self.contains(Self::WRITE) {
            parts.push("write");
        }
        if self.contains(Self::DELETE) {
            parts.push("delete");
        }
        if self.contains(Self::ADMIN) {
            parts.push("admin");
        }
        if self.contains(Self::MANAGE_USERS) {
            parts.push("manage_users");
        }
        if self.contains(Self::MANAGE_ROLES) {
            parts.push("manage_roles");
        }
        if self.contains(Self::BILLING) {
            parts.push("billing");
        }
        if self.contains(Self::AUDIT) {
            parts.push("audit");
        }
        if self.contains(Self::EXPORT) {
            parts.push("export");
        }
        if self.contains(Self::IMPORT) {
            parts.push("import");
        }
        if self.contains(Self::API_ACCESS) {
            parts.push("api_access");
        }
        if self.contains(Self::PREMIUM) {
            parts.push("premium");
        }

        write!(f, "{}", parts.join(", "))
    }
}

/// Trait for types that represent a role with permissions.
///
/// Implement this trait for custom role enums to integrate
/// with the permission system.
///
/// # Examples
///
/// ```rust
/// use revelation_user::{Permissions, Role};
///
/// #[derive(Debug, Clone, Copy)]
/// enum AppRole {
///     Guest,
///     Member,
///     Admin
/// }
///
/// impl Role for AppRole {
///     fn permissions(&self) -> Permissions {
///         match self {
///             Self::Guest => Permissions::READ,
///             Self::Member => Permissions::READ | Permissions::WRITE | Permissions::PREMIUM,
///             Self::Admin => Permissions::all()
///         }
///     }
///
///     fn name(&self) -> &'static str {
///         match self {
///             Self::Guest => "guest",
///             Self::Member => "member",
///             Self::Admin => "admin"
///         }
///     }
/// }
///
/// let admin = AppRole::Admin;
/// assert!(admin.can(Permissions::ADMIN));
/// assert!(admin.can_all(Permissions::READ | Permissions::WRITE));
/// ```
pub trait Role: Send + Sync {
    /// Get the permissions associated with this role.
    fn permissions(&self) -> Permissions;

    /// Get the role name for display/logging.
    fn name(&self) -> &'static str;

    /// Check if this role has the specified permission.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{Permissions, RUserRole, Role};
    ///
    /// let admin = RUserRole::Admin;
    /// assert!(admin.can(Permissions::DELETE));
    /// ```
    #[inline]
    fn can(&self, permission: Permissions) -> bool {
        self.permissions().contains(permission)
    }

    /// Check if this role has all the specified permissions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{Permissions, RUserRole, Role};
    ///
    /// let admin = RUserRole::Admin;
    /// let required = Permissions::READ | Permissions::WRITE | Permissions::DELETE;
    /// assert!(admin.can_all(required));
    /// ```
    #[inline]
    fn can_all(&self, permissions: Permissions) -> bool {
        self.permissions().contains(permissions)
    }

    /// Check if this role has any of the specified permissions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::{Permissions, RUserRole, Role};
    ///
    /// let user = RUserRole::User;
    /// let any_of = Permissions::ADMIN | Permissions::READ;
    /// assert!(user.can_any(any_of)); // Has READ
    /// ```
    #[inline]
    fn can_any(&self, permissions: Permissions) -> bool {
        self.permissions().intersects(permissions)
    }

    /// Check if this role is an admin role.
    ///
    /// Default implementation checks for ADMIN permission.
    #[inline]
    fn is_admin(&self) -> bool {
        self.can(Permissions::ADMIN)
    }

    /// Check if this role has premium access.
    ///
    /// Default implementation checks for PREMIUM permission.
    #[inline]
    fn is_premium(&self) -> bool {
        self.can(Permissions::PREMIUM)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permissions_bitwise_operations() {
        let read_write = Permissions::READ | Permissions::WRITE;
        assert!(read_write.contains(Permissions::READ));
        assert!(read_write.contains(Permissions::WRITE));
        assert!(!read_write.contains(Permissions::DELETE));
    }

    #[test]
    fn permissions_satisfies() {
        let perms = Permissions::READ | Permissions::WRITE | Permissions::DELETE;
        assert!(perms.satisfies(Permissions::READ));
        assert!(perms.satisfies(Permissions::READ | Permissions::WRITE));
        assert!(!perms.satisfies(Permissions::ADMIN));
    }

    #[test]
    fn permissions_from_bits_checked() {
        let valid = Permissions::from_bits_checked(0x0003);
        assert_eq!(valid, Some(Permissions::READ | Permissions::WRITE));

        let invalid = Permissions::from_bits_checked(0xFFFF_0000);
        assert!(invalid.is_none());
    }

    #[test]
    fn permissions_from_bits_truncating() {
        let perms = Permissions::from_bits_truncating(0xFFFF_FFFF);
        assert_eq!(perms, Permissions::all());
    }

    #[test]
    fn permissions_is_none() {
        assert!(Permissions::empty().is_none());
        assert!(!Permissions::READ.is_none());
    }

    #[test]
    fn permissions_as_u32() {
        let perms = Permissions::READ | Permissions::WRITE;
        assert_eq!(perms.as_u32(), 0x0003);
    }

    #[test]
    fn permissions_default() {
        assert_eq!(Permissions::default(), Permissions::READ);
    }

    #[test]
    fn permissions_display_empty() {
        assert_eq!(format!("{}", Permissions::empty()), "none");
    }

    #[test]
    fn permissions_display_single() {
        assert_eq!(format!("{}", Permissions::READ), "read");
        assert_eq!(format!("{}", Permissions::ADMIN), "admin");
    }

    #[test]
    fn permissions_display_multiple() {
        let perms = Permissions::READ | Permissions::WRITE;
        assert_eq!(format!("{}", perms), "read, write");
    }

    #[test]
    fn permissions_display_all() {
        let display = format!("{}", Permissions::all());
        assert!(display.contains("read"));
        assert!(display.contains("admin"));
        assert!(display.contains("premium"));
    }

    #[test]
    fn permissions_serializes_as_number() {
        let perms = Permissions::READ | Permissions::WRITE;
        let json = serde_json::to_string(&perms).unwrap();
        assert_eq!(json, "3");
    }

    #[test]
    fn permissions_deserializes_from_number() {
        let perms: Permissions = serde_json::from_str("3").unwrap();
        assert_eq!(perms, Permissions::READ | Permissions::WRITE);
    }

    #[test]
    fn permissions_deserializes_from_string() {
        let perms: Permissions = serde_json::from_str("\"read, write\"").unwrap();
        assert_eq!(perms, Permissions::READ | Permissions::WRITE);
    }

    #[test]
    fn permissions_deserializes_from_pipe_string() {
        let perms: Permissions = serde_json::from_str("\"READ | WRITE\"").unwrap();
        assert_eq!(perms, Permissions::READ | Permissions::WRITE);
    }

    #[test]
    fn permissions_roundtrip() {
        let original = Permissions::READ | Permissions::WRITE | Permissions::DELETE;
        let json = serde_json::to_string(&original).unwrap();
        let restored: Permissions = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn permissions_deserialize_all_names() {
        let perms: Permissions =
            serde_json::from_str("\"admin, manage_users, manage_roles, billing, audit\"").unwrap();
        assert!(perms.contains(Permissions::ADMIN));
        assert!(perms.contains(Permissions::MANAGE_USERS));
        assert!(perms.contains(Permissions::MANAGE_ROLES));
        assert!(perms.contains(Permissions::BILLING));
        assert!(perms.contains(Permissions::AUDIT));

        let perms2: Permissions =
            serde_json::from_str("\"export, import, api_access, premium, delete\"").unwrap();
        assert!(perms2.contains(Permissions::EXPORT));
        assert!(perms2.contains(Permissions::IMPORT));
        assert!(perms2.contains(Permissions::API_ACCESS));
        assert!(perms2.contains(Permissions::PREMIUM));
        assert!(perms2.contains(Permissions::DELETE));
    }

    #[test]
    fn permissions_deserialize_empty_string() {
        let perms: Permissions = serde_json::from_str("\"\"").unwrap();
        assert!(perms.is_empty());
    }

    #[test]
    fn permissions_deserialize_invalid_name() {
        let result: Result<Permissions, _> = serde_json::from_str("\"invalid_perm\"");
        assert!(result.is_err());
    }

    #[test]
    fn permissions_deserialize_negative_number() {
        let result: Result<Permissions, _> = serde_json::from_str("-1");
        assert!(result.is_err());
    }

    #[test]
    fn permissions_deserialize_invalid_bits() {
        let result: Result<Permissions, _> = serde_json::from_str("4294967295");
        assert!(result.is_err());
    }

    #[test]
    fn permissions_deserialize_from_i64() {
        // Test positive i64 path
        let perms: Permissions = serde_json::from_value(serde_json::json!(3i64)).unwrap();
        assert_eq!(perms, Permissions::READ | Permissions::WRITE);
    }

    #[test]
    fn parse_permissions_with_write() {
        let perms = super::parse_permissions("write").unwrap();
        assert_eq!(perms, Permissions::WRITE);
    }

    #[test]
    fn permissions_presets() {
        assert_eq!(Permissions::VIEWER, Permissions::READ);
        assert_eq!(Permissions::EDITOR, Permissions::READ | Permissions::WRITE);
        assert!(Permissions::MANAGER.contains(Permissions::MANAGE_USERS));
        assert!(Permissions::MANAGER.contains(Permissions::DELETE));
    }

    #[derive(Debug, Clone, Copy)]
    enum TestRole {
        Guest,
        Member,
        Admin
    }

    impl Role for TestRole {
        fn permissions(&self) -> Permissions {
            match self {
                Self::Guest => Permissions::READ,
                Self::Member => Permissions::READ | Permissions::WRITE | Permissions::PREMIUM,
                Self::Admin => Permissions::all()
            }
        }

        fn name(&self) -> &'static str {
            match self {
                Self::Guest => "guest",
                Self::Member => "member",
                Self::Admin => "admin"
            }
        }
    }

    #[test]
    fn role_trait_can() {
        let admin = TestRole::Admin;
        assert!(admin.can(Permissions::DELETE));
        assert!(admin.can(Permissions::ADMIN));

        let guest = TestRole::Guest;
        assert!(guest.can(Permissions::READ));
        assert!(!guest.can(Permissions::WRITE));
    }

    #[test]
    fn role_trait_can_all() {
        let admin = TestRole::Admin;
        let required = Permissions::READ | Permissions::WRITE | Permissions::DELETE;
        assert!(admin.can_all(required));

        let guest = TestRole::Guest;
        assert!(!guest.can_all(required));
    }

    #[test]
    fn role_trait_can_any() {
        let guest = TestRole::Guest;
        let any_of = Permissions::ADMIN | Permissions::READ;
        assert!(guest.can_any(any_of));

        let no_match = Permissions::ADMIN | Permissions::DELETE;
        assert!(!guest.can_any(no_match));
    }

    #[test]
    fn role_trait_is_admin() {
        assert!(TestRole::Admin.is_admin());
        assert!(!TestRole::Member.is_admin());
        assert!(!TestRole::Guest.is_admin());
    }

    #[test]
    fn role_trait_is_premium() {
        assert!(TestRole::Admin.is_premium());
        assert!(TestRole::Member.is_premium());
        assert!(!TestRole::Guest.is_premium());
    }

    #[test]
    fn role_trait_name() {
        assert_eq!(TestRole::Guest.name(), "guest");
        assert_eq!(TestRole::Member.name(), "member");
        assert_eq!(TestRole::Admin.name(), "admin");
    }

    #[test]
    fn permissions_deserialize_unexpected_type_triggers_expecting() {
        // Boolean triggers the expecting() method for a better error message
        let result: Result<Permissions, _> = serde_json::from_str("true");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("number") || err.contains("string"));
    }

    #[test]
    fn permissions_deserialize_negative_i64_via_value() {
        // Use serde_json::Value to ensure we go through visit_i64 path
        use serde::Deserialize;
        let negative = serde_json::json!(-42);
        let result = Permissions::deserialize(&negative);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("negative"));
    }

    #[test]
    fn permissions_deserialize_positive_i64_via_serde_test() {
        // serde_test::Token::I64 forces the visit_i64 path with positive value
        use serde_test::{Token, assert_de_tokens};
        assert_de_tokens(&Permissions::READ, &[Token::I64(1)]);
        assert_de_tokens(&(Permissions::READ | Permissions::WRITE), &[Token::I64(3)]);
        // Zero i64 path
        assert_de_tokens(&Permissions::empty(), &[Token::I64(0)]);
    }
}
