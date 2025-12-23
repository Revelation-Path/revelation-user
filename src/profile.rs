use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::Gender;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserProfile {
    #[validate(length(min = 2, max = 100))]
    pub name:          Option<String>,
    pub gender:        Option<Gender>,
    pub birth_date:    Option<NaiveDate>,
    pub confession_id: Option<Uuid>
}
