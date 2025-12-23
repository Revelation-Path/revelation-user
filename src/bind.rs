use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct BindTelegram {
    pub telegram_id: i64
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct BindEmail {
    #[validate(email)]
    pub email: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct BindPhone {
    #[validate(length(min = 10, max = 15))]
    pub phone: String
}
