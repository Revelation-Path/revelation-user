//! Gender enumeration.

use serde::{Deserialize, Serialize};

/// User's gender.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(feature = "db", sqlx(type_name = "gender", rename_all = "snake_case"))]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub enum Gender {
    Male,
    Female
}
