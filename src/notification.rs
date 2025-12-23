//! Notification-related types.

use serde::{Deserialize, Serialize};

/// Telegram notification recipient.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramRecipient {
    /// Telegram chat ID.
    pub chat_id: i64
}
