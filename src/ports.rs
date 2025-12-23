//! Port traits for infrastructure layer.

use std::future::Future;

use masterror::AppResult;

use crate::TelegramRecipient;

/// Repository for notification operations.
pub trait NotificationRepository: Send + Sync {
    /// Get all Telegram notification recipients.
    fn get_telegram_recipients(
        &self
    ) -> impl Future<Output = AppResult<Vec<TelegramRecipient>>> + Send;
}
