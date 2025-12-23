use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::Gender;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id:            Uuid,
    pub name:          Option<String>,
    pub gender:        Option<Gender>,
    pub birth_date:    Option<NaiveDate>,
    pub confession_id: Option<Uuid>,
    pub email:         Option<String>,
    pub phone:         Option<String>,
    pub telegram_id:   Option<i64>,
    pub created_at:    DateTime<Utc>,
    pub updated_at:    DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUser {
    pub id: Uuid
}
