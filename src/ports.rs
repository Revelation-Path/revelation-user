//! Port traits for infrastructure layer.
//!
//! This module defines repository traits following hexagonal architecture
//! (ports and adapters pattern). Ports define the interface that the
//! domain layer expects, while adapters provide concrete implementations.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────┐
//! │  Domain Layer   │
//! │  (this crate)   │
//! │                 │
//! │  ┌───────────┐  │
//! │  │   Ports   │  │  <- Trait definitions
//! │  └───────────┘  │
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ Infrastructure  │
//! │                 │
//! │  ┌───────────┐  │
//! │  │ Adapters  │  │  <- Implementations (sqlx, etc.)
//! │  └───────────┘  │
//! └─────────────────┘
//! ```
//!
//! # Overview
//!
//! | Trait | Purpose |
//! |-------|---------|
//! | [`NotificationRepository`] | Load notification recipients |
//!
//! # Design Principles
//!
//! - **Dependency Inversion**: Domain depends on abstractions, not concretions
//! - **Async-First**: All methods return futures for non-blocking I/O
//! - **Error Handling**: Unified [`AppResult`] type for consistent error
//!   handling
//!
//! # Examples
//!
//! ## Defining an Implementation
//!
//! ```rust,ignore
//! use revelation_user::{TelegramRecipient, ports::NotificationRepository};
//! use masterror::AppResult;
//! use sqlx::PgPool;
//!
//! struct PostgresNotificationRepo {
//!     pool: PgPool,
//! }
//!
//! impl NotificationRepository for PostgresNotificationRepo {
//!     async fn get_telegram_recipients(&self) -> AppResult<Vec<TelegramRecipient>> {
//!         sqlx::query_as!(
//!             TelegramRecipient,
//!             "SELECT chat_id FROM telegram_recipients WHERE active = true"
//!         )
//!         .fetch_all(&self.pool)
//!         .await
//!         .map_err(Into::into)
//!     }
//! }
//! ```
//!
//! ## Using with Dependency Injection
//!
//! ```rust,ignore
//! use revelation_user::ports::NotificationRepository;
//!
//! struct NotificationService<R: NotificationRepository> {
//!     repo: R,
//! }
//!
//! impl<R: NotificationRepository> NotificationService<R> {
//!     async fn broadcast(&self, message: &str) -> AppResult<usize> {
//!         let recipients = self.repo.get_telegram_recipients().await?;
//!         // Send to each recipient...
//!         Ok(recipients.len())
//!     }
//! }
//! ```
//!
//! [`AppResult`]: masterror::AppResult

use std::future::Future;

use masterror::AppResult;

use crate::TelegramRecipient;

/// Repository trait for notification operations.
///
/// This trait defines the interface for loading notification
/// recipients from persistent storage.
///
/// # Implementors
///
/// Infrastructure layer should provide implementations backed by:
/// - PostgreSQL via sqlx
/// - In-memory store for testing
/// - Other databases as needed
///
/// # Thread Safety
///
/// Implementations must be `Send + Sync` to support concurrent access
/// from multiple async tasks.
///
/// # Examples
///
/// ## PostgreSQL Implementation
///
/// ```rust,ignore
/// use revelation_user::{TelegramRecipient, ports::NotificationRepository};
/// use masterror::AppResult;
/// use sqlx::PgPool;
///
/// pub struct PgNotificationRepo {
///     pool: PgPool,
/// }
///
/// impl NotificationRepository for PgNotificationRepo {
///     async fn get_telegram_recipients(&self) -> AppResult<Vec<TelegramRecipient>> {
///         sqlx::query_as!(
///             TelegramRecipient,
///             r#"
///             SELECT chat_id
///             FROM telegram_recipients
///             WHERE notifications_enabled = true
///             "#
///         )
///         .fetch_all(&self.pool)
///         .await
///         .map_err(Into::into)
///     }
/// }
/// ```
///
/// ## Mock Implementation for Testing
///
/// ```rust
/// use masterror::AppResult;
/// use revelation_user::{TelegramRecipient, ports::NotificationRepository};
///
/// struct MockNotificationRepo {
///     recipients: Vec<TelegramRecipient>
/// }
///
/// impl NotificationRepository for MockNotificationRepo {
///     async fn get_telegram_recipients(&self) -> AppResult<Vec<TelegramRecipient>> {
///         Ok(self.recipients.clone())
///     }
/// }
///
/// // Usage in tests
/// let mock = MockNotificationRepo {
///     recipients: vec![TelegramRecipient::new(111), TelegramRecipient::new(222)]
/// };
/// ```
///
/// ## Generic Service
///
/// ```rust,ignore
/// use revelation_user::ports::NotificationRepository;
///
/// async fn send_notifications<R: NotificationRepository>(
///     repo: &R,
///     message: &str,
/// ) -> AppResult<()> {
///     for recipient in repo.get_telegram_recipients().await? {
///         // Send via Telegram API
///         telegram.send(recipient.chat_id, message).await?;
///     }
///     Ok(())
/// }
/// ```
pub trait NotificationRepository: Send + Sync {
    /// Retrieve all active Telegram notification recipients.
    ///
    /// Returns a list of Telegram chat IDs that should receive
    /// notifications. Implementations should filter out disabled
    /// or inactive recipients.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<TelegramRecipient>)` - List of recipients
    /// - `Err(AppError)` - Database or other infrastructure error
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use revelation_user::ports::NotificationRepository;
    ///
    /// async fn count_recipients(repo: &impl NotificationRepository) -> usize {
    ///     repo.get_telegram_recipients()
    ///         .await
    ///         .map(|r| r.len())
    ///         .unwrap_or(0)
    /// }
    /// ```
    fn get_telegram_recipients(
        &self
    ) -> impl Future<Output = AppResult<Vec<TelegramRecipient>>> + Send;
}
