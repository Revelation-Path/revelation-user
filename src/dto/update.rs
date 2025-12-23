//! User profile update DTOs.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::Gender;

/// Request to update user profile.
///
/// All fields are optional - only provided fields will be updated.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct UpdateProfileRequest {
    /// Display name (2-100 characters).
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,

    /// User's gender.
    pub gender: Option<Gender>,

    /// Date of birth.
    pub birth_date: Option<NaiveDate>,

    /// Confession/denomination ID.
    pub confession_id: Option<Uuid>
}

impl UpdateProfileRequest {
    /// Check if request has any updates.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.gender.is_none()
            && self.birth_date.is_none()
            && self.confession_id.is_none()
    }
}
