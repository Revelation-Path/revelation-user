//! User profile update DTOs.
//!
//! This module provides request types for updating user profile data.
//!
//! # Overview
//!
//! [`UpdateProfileRequest`] contains all updateable profile fields.
//! All fields are optional - only provided fields will be updated.
//!
//! # Examples
//!
//! ```rust
//! use revelation_user::{Gender, UpdateProfileRequest};
//! use validator::Validate;
//!
//! let req = UpdateProfileRequest {
//!     name:          Some("John Doe".into()),
//!     gender:        Some(Gender::Male),
//!     birth_date:    None,
//!     confession_id: None
//! };
//!
//! assert!(req.validate().is_ok());
//! assert!(!req.is_empty());
//! ```

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::Gender;

/// Request to update user profile fields.
///
/// All fields are optional. Only fields with `Some` value
/// will be updated; `None` means "don't change".
///
/// # Validation
///
/// - `name`: Must be 2-100 characters when provided
///
/// # Examples
///
/// ## Partial Update
///
/// ```rust
/// use revelation_user::UpdateProfileRequest;
///
/// // Only update name
/// let req = UpdateProfileRequest {
///     name:          Some("New Name".into()),
///     gender:        None,
///     birth_date:    None,
///     confession_id: None
/// };
///
/// assert!(!req.is_empty());
/// ```
///
/// ## Full Update
///
/// ```rust
/// use chrono::NaiveDate;
/// use revelation_user::{Gender, UpdateProfileRequest};
/// use uuid::Uuid;
///
/// let req = UpdateProfileRequest {
///     name:          Some("John Doe".into()),
///     gender:        Some(Gender::Male),
///     birth_date:    Some(NaiveDate::from_ymd_opt(1990, 1, 15).unwrap()),
///     confession_id: Some(Uuid::now_v7())
/// };
/// ```
///
/// ## Validation Error
///
/// ```rust
/// use revelation_user::UpdateProfileRequest;
/// use validator::Validate;
///
/// let req = UpdateProfileRequest {
///     name:          Some("X".into()), // Too short!
///     gender:        None,
///     birth_date:    None,
///     confession_id: None
/// };
///
/// assert!(req.validate().is_err());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct UpdateProfileRequest {
    /// Display name (2-100 characters).
    ///
    /// When provided, must be between 2 and 100 characters.
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,

    /// User's gender.
    pub gender: Option<Gender>,

    /// Date of birth.
    pub birth_date: Option<NaiveDate>,

    /// Reference to confession/denomination.
    pub confession_id: Option<Uuid>
}

impl UpdateProfileRequest {
    /// Check if the request has any updates.
    ///
    /// Returns `true` if all fields are `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::UpdateProfileRequest;
    ///
    /// let empty = UpdateProfileRequest {
    ///     name:          None,
    ///     gender:        None,
    ///     birth_date:    None,
    ///     confession_id: None
    /// };
    /// assert!(empty.is_empty());
    ///
    /// let not_empty = UpdateProfileRequest {
    ///     name:          Some("Test".into()),
    ///     gender:        None,
    ///     birth_date:    None,
    ///     confession_id: None
    /// };
    /// assert!(!not_empty.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.gender.is_none()
            && self.birth_date.is_none()
            && self.confession_id.is_none()
    }

    /// Create an empty update request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::UpdateProfileRequest;
    ///
    /// let req = UpdateProfileRequest::empty();
    /// assert!(req.is_empty());
    /// ```
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            name:          None,
            gender:        None,
            birth_date:    None,
            confession_id: None
        }
    }
}

impl Default for UpdateProfileRequest {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty_when_all_none() {
        let req = UpdateProfileRequest::empty();
        assert!(req.is_empty());
    }

    #[test]
    fn not_empty_when_has_name() {
        let req = UpdateProfileRequest {
            name: Some("Test".into()),
            ..Default::default()
        };
        assert!(!req.is_empty());
    }

    #[test]
    fn validates_name_length() {
        let too_short = UpdateProfileRequest {
            name: Some("X".into()),
            ..Default::default()
        };
        assert!(too_short.validate().is_err());

        let valid = UpdateProfileRequest {
            name: Some("Valid Name".into()),
            ..Default::default()
        };
        assert!(valid.validate().is_ok());
    }

    #[test]
    fn default_is_empty() {
        let req = UpdateProfileRequest::default();
        assert!(req.is_empty());
    }
}
