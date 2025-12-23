use serde::{Deserialize, Serialize};

/// Recipient for Telegram notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramRecipient {
    pub chat_id: i64
}
