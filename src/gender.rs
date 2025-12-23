//! Gender enumeration for user profiles.
//!
//! This module provides the [`Gender`] enum for representing
//! user gender in the system.
//!
//! # Database Integration
//!
//! With the `db` feature, [`Gender`] maps to PostgreSQL enum type:
//!
//! ```sql
//! CREATE TYPE gender AS ENUM ('male', 'female');
//! ```
//!
//! # Serialization
//!
//! Values are serialized as snake_case strings:
//!
//! ```rust
//! use revelation_user::Gender;
//!
//! let json = serde_json::to_string(&Gender::Male).unwrap();
//! assert_eq!(json, "\"male\"");
//!
//! let json = serde_json::to_string(&Gender::Female).unwrap();
//! assert_eq!(json, "\"female\"");
//! ```

use serde::{Deserialize, Serialize};

/// User's biological gender.
///
/// Used for personalization and analytics.
///
/// # Serialization
///
/// Serializes to lowercase snake_case:
/// - `Gender::Male` → `"male"`
/// - `Gender::Female` → `"female"`
///
/// # Database
///
/// With `db` feature, maps to PostgreSQL `gender` enum type.
///
/// # Examples
///
/// ```rust
/// use revelation_user::Gender;
///
/// let gender = Gender::Male;
///
/// // Serialization
/// let json = serde_json::to_string(&gender).unwrap();
/// assert_eq!(json, "\"male\"");
///
/// // Deserialization
/// let parsed: Gender = serde_json::from_str("\"female\"").unwrap();
/// assert_eq!(parsed, Gender::Female);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(feature = "db", sqlx(type_name = "gender", rename_all = "snake_case"))]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub enum Gender {
    /// Male gender.
    Male,

    /// Female gender.
    Female
}

impl Gender {
    /// Returns `true` if this is `Gender::Male`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::Gender;
    ///
    /// assert!(Gender::Male.is_male());
    /// assert!(!Gender::Female.is_male());
    /// ```
    #[must_use]
    pub const fn is_male(&self) -> bool {
        matches!(self, Self::Male)
    }

    /// Returns `true` if this is `Gender::Female`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::Gender;
    ///
    /// assert!(Gender::Female.is_female());
    /// assert!(!Gender::Male.is_female());
    /// ```
    #[must_use]
    pub const fn is_female(&self) -> bool {
        matches!(self, Self::Female)
    }

    /// Returns the gender as a lowercase string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::Gender;
    ///
    /// assert_eq!(Gender::Male.as_str(), "male");
    /// assert_eq!(Gender::Female.as_str(), "female");
    /// ```
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Male => "male",
            Self::Female => "female"
        }
    }
}

impl core::fmt::Display for Gender {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_to_snake_case() {
        assert_eq!(serde_json::to_string(&Gender::Male).unwrap(), "\"male\"");
        assert_eq!(
            serde_json::to_string(&Gender::Female).unwrap(),
            "\"female\""
        );
    }

    #[test]
    fn deserializes_from_snake_case() {
        assert_eq!(
            serde_json::from_str::<Gender>("\"male\"").unwrap(),
            Gender::Male
        );
        assert_eq!(
            serde_json::from_str::<Gender>("\"female\"").unwrap(),
            Gender::Female
        );
    }

    #[test]
    fn display_impl() {
        assert_eq!(format!("{}", Gender::Male), "male");
        assert_eq!(format!("{}", Gender::Female), "female");
    }

    #[test]
    fn is_methods() {
        assert!(Gender::Male.is_male());
        assert!(!Gender::Male.is_female());
        assert!(Gender::Female.is_female());
        assert!(!Gender::Female.is_male());
    }
}
