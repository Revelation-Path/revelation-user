//! Public user projection - safe to expose in API responses.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Gender, RUser};

/// Public user data safe for API responses.
///
/// Excludes sensitive fields like email, phone, telegram_id.
///
/// # Examples
///
/// ```
/// use revelation_user::{RUser, RUserPublic};
///
/// let user = RUser::from_telegram(123456);
/// let public: RUserPublic = user.into();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct RUserPublic {
    /// User ID.
    pub id: Uuid,

    /// Display name.
    pub name: Option<String>,

    /// User's gender.
    pub gender: Option<Gender>
}

impl From<RUser> for RUserPublic {
    fn from(user: RUser) -> Self {
        Self {
            id:     user.id,
            name:   user.name,
            gender: user.gender
        }
    }
}

impl From<&RUser> for RUserPublic {
    fn from(user: &RUser) -> Self {
        Self {
            id:     user.id,
            name:   user.name.clone(),
            gender: user.gender
        }
    }
}
