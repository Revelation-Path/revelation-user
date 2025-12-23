//! Notification recipient types.
//!
//! This module provides types for notification delivery targets.
//! Currently supports Telegram notifications with planned expansion
//! to email and push notifications.
//!
//! # Overview
//!
//! | Type | Description |
//! |------|-------------|
//! | [`TelegramRecipient`] | Telegram chat/user as notification target |
//!
//! # Use Cases
//!
//! - **Broadcast Notifications**: Send updates to subscribed users
//! - **Alerts**: Notify admins of system events
//! - **Reminders**: Scheduled notifications to users
//!
//! # Examples
//!
//! ```rust
//! use revelation_user::TelegramRecipient;
//!
//! // Create a recipient
//! let recipient = TelegramRecipient {
//!     chat_id: 123456789
//! };
//!
//! // Serialize for storage
//! let json = serde_json::to_string(&recipient).unwrap();
//! assert!(json.contains("123456789"));
//!
//! // Deserialize from database
//! let loaded: TelegramRecipient = serde_json::from_str(&json).unwrap();
//! assert_eq!(loaded.chat_id, 123456789);
//! ```
//!
//! # Integration
//!
//! Recipients are typically loaded via [`NotificationRepository`]
//! and then used with a Telegram bot API to send messages.
//!
//! ```rust,ignore
//! use revelation_user::{TelegramRecipient, ports::NotificationRepository};
//!
//! async fn send_broadcast(repo: &impl NotificationRepository, message: &str) {
//!     let recipients = repo.get_telegram_recipients().await.unwrap();
//!
//!     for recipient in recipients {
//!         // Send via Telegram bot API
//!         telegram_bot.send_message(recipient.chat_id, message).await;
//!     }
//! }
//! ```
//!
//! [`NotificationRepository`]: crate::ports::NotificationRepository

use serde::{Deserialize, Serialize};

/// Telegram notification recipient.
///
/// Represents a Telegram user or chat that can receive
/// notifications from the system.
///
/// # Fields
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `chat_id` | `i64` | Telegram chat/user ID |
///
/// # Chat ID Format
///
/// Telegram chat IDs can be:
/// - **Positive**: Regular users (e.g., `123456789`)
/// - **Negative**: Groups and channels (e.g., `-1001234567890`)
///
/// # Examples
///
/// ## Creating a Recipient
///
/// ```rust
/// use revelation_user::TelegramRecipient;
///
/// // User recipient
/// let user = TelegramRecipient {
///     chat_id: 123456789
/// };
///
/// // Group recipient (negative ID)
/// let group = TelegramRecipient {
///     chat_id: -1001234567890
/// };
/// ```
///
/// ## Serialization
///
/// ```rust
/// use revelation_user::TelegramRecipient;
///
/// let recipient = TelegramRecipient {
///     chat_id: 123456789
/// };
///
/// // To JSON
/// let json = serde_json::to_string(&recipient).unwrap();
/// assert_eq!(json, r#"{"chat_id":123456789}"#);
///
/// // From JSON
/// let parsed: TelegramRecipient = serde_json::from_str(&json).unwrap();
/// assert_eq!(parsed.chat_id, recipient.chat_id);
/// ```
///
/// ## With Vector of Recipients
///
/// ```rust
/// use revelation_user::TelegramRecipient;
///
/// let recipients = vec![
///     TelegramRecipient {
///         chat_id: 111111
///     },
///     TelegramRecipient {
///         chat_id: 222222
///     },
///     TelegramRecipient {
///         chat_id: 333333
///     },
/// ];
///
/// for recipient in &recipients {
///     println!("Sending to chat: {}", recipient.chat_id);
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TelegramRecipient {
    /// Telegram chat or user ID.
    ///
    /// - Positive values: Individual users
    /// - Negative values: Groups and channels
    ///
    /// This ID is obtained from Telegram bot callbacks or
    /// when a user starts interaction with the bot.
    pub chat_id: i64
}

impl TelegramRecipient {
    /// Create a new Telegram recipient.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Telegram chat or user ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::TelegramRecipient;
    ///
    /// let recipient = TelegramRecipient::new(123456789);
    /// assert_eq!(recipient.chat_id, 123456789);
    /// ```
    #[must_use]
    pub const fn new(chat_id: i64) -> Self {
        Self {
            chat_id
        }
    }

    /// Check if this is a user (positive ID) vs group (negative ID).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::TelegramRecipient;
    ///
    /// let user = TelegramRecipient::new(123456789);
    /// assert!(user.is_user());
    ///
    /// let group = TelegramRecipient::new(-1001234567890);
    /// assert!(!group.is_user());
    /// ```
    #[must_use]
    pub const fn is_user(&self) -> bool {
        self.chat_id > 0
    }

    /// Check if this is a group/channel (negative ID).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::TelegramRecipient;
    ///
    /// let group = TelegramRecipient::new(-1001234567890);
    /// assert!(group.is_group());
    ///
    /// let user = TelegramRecipient::new(123456789);
    /// assert!(!user.is_group());
    /// ```
    #[must_use]
    pub const fn is_group(&self) -> bool {
        self.chat_id < 0
    }
}

impl From<i64> for TelegramRecipient {
    /// Create a recipient from a chat ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use revelation_user::TelegramRecipient;
    ///
    /// let recipient: TelegramRecipient = 123456789_i64.into();
    /// assert_eq!(recipient.chat_id, 123456789);
    /// ```
    fn from(chat_id: i64) -> Self {
        Self::new(chat_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_recipient() {
        let recipient = TelegramRecipient::new(123456789);
        assert_eq!(recipient.chat_id, 123456789);
    }

    #[test]
    fn is_user_for_positive_id() {
        let user = TelegramRecipient::new(123456789);
        assert!(user.is_user());
        assert!(!user.is_group());
    }

    #[test]
    fn is_group_for_negative_id() {
        let group = TelegramRecipient::new(-1001234567890);
        assert!(group.is_group());
        assert!(!group.is_user());
    }

    #[test]
    fn from_i64_conversion() {
        let recipient: TelegramRecipient = 123456789_i64.into();
        assert_eq!(recipient.chat_id, 123456789);
    }

    #[test]
    fn serialization_roundtrip() {
        let original = TelegramRecipient::new(123456789);
        let json = serde_json::to_string(&original).unwrap();
        let parsed: TelegramRecipient = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn copy_semantics() {
        let original = TelegramRecipient::new(123);
        let copied = original;
        assert_eq!(original.chat_id, copied.chat_id);
    }
}
