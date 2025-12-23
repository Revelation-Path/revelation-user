use masterror::AppResult;

use crate::TelegramRecipient;

/// Repository for notification-related operations
pub trait NotificationRepository {
    /// Get all users who want Telegram notifications
    fn get_telegram_recipients(
        &self
    ) -> impl Future<Output = AppResult<Vec<TelegramRecipient>>> + Send;
}

use std::future::Future;
