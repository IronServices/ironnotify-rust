//! IronNotify SDK for Rust
//!
//! Event notifications and alerts SDK for Rust applications.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use ironnotify::{NotifyClient, NotifyOptions};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Initialize
//!     let client = NotifyClient::new(NotifyOptions::new("ak_live_xxxxx"))
//!         .expect("Failed to create client");
//!
//!     // Send a simple notification
//!     let result = client.notify("order.created", "New Order Received").await;
//!
//!     if result.success {
//!         println!("Notification sent!");
//!     }
//! }
//! ```
//!
//! # Event Builder
//!
//! ```rust,no_run
//! use ironnotify::{NotifyClient, NotifyOptions, SeverityLevel, NotificationAction};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = NotifyClient::new(NotifyOptions::new("ak_live_xxxxx"))
//!         .expect("Failed to create client");
//!
//!     let result = client.event("payment.failed")
//!         .with_title("Payment Failed")
//!         .with_message("Payment could not be processed")
//!         .with_severity(SeverityLevel::Error)
//!         .with_metadata("order_id", "1234")
//!         .with_url_action("Retry Payment", "/orders/1234/retry")
//!         .for_user("user-123")
//!         .expires_in_std(Duration::from_secs(86400))
//!         .send()
//!         .await;
//!
//!     if result.queued {
//!         println!("Notification queued for later");
//!     }
//! }
//! ```

mod builder;
mod client;
mod config;
mod queue;
mod transport;
mod types;

pub use builder::EventBuilder;
pub use client::NotifyClient;
pub use config::{NotifyOptions, NotifyOptionsBuilder};
pub use types::{
    ConnectionState, Notification, NotificationAction, NotificationPayload, SendResult,
    SeverityLevel,
};

use once_cell::sync::OnceCell;
use std::sync::Arc;

static GLOBAL_CLIENT: OnceCell<Arc<NotifyClient>> = OnceCell::new();

/// Initializes the global client with an API key.
pub fn init(api_key: impl Into<String>) -> Result<(), &'static str> {
    init_with_options(NotifyOptions::new(api_key))
}

/// Initializes the global client with options.
pub fn init_with_options(options: NotifyOptions) -> Result<(), &'static str> {
    let client = NotifyClient::new(options)?;
    GLOBAL_CLIENT.set(client).map_err(|_| "Already initialized")
}

/// Gets the global client.
pub fn get_client() -> Result<&'static Arc<NotifyClient>, &'static str> {
    GLOBAL_CLIENT.get().ok_or("Not initialized. Call init() first.")
}

/// Sends a notification using the global client.
pub async fn notify(
    event_type: impl Into<String>,
    title: impl Into<String>,
) -> Result<SendResult, &'static str> {
    let client = get_client()?;
    Ok(client.notify(event_type, title).await)
}

/// Creates an event builder using the global client.
pub fn event(event_type: impl Into<String>) -> Result<EventBuilder, &'static str> {
    let client = get_client()?;
    Ok(client.event(event_type))
}

/// Gets notifications using the global client.
pub async fn get_notifications(
    limit: Option<i32>,
    offset: Option<i32>,
    unread_only: bool,
) -> Result<Vec<Notification>, String> {
    let client = get_client().map_err(|e| e.to_string())?;
    client.get_notifications(limit, offset, unread_only).await
}

/// Gets the unread count using the global client.
pub async fn get_unread_count() -> Result<i32, String> {
    let client = get_client().map_err(|e| e.to_string())?;
    client.get_unread_count().await
}

/// Marks a notification as read using the global client.
pub async fn mark_as_read(notification_id: &str) -> Result<bool, String> {
    let client = get_client().map_err(|e| e.to_string())?;
    client.mark_as_read(notification_id).await
}

/// Marks all notifications as read using the global client.
pub async fn mark_all_as_read() -> Result<bool, String> {
    let client = get_client().map_err(|e| e.to_string())?;
    client.mark_all_as_read().await
}

/// Flushes the offline queue using the global client.
pub async fn flush() -> Result<(), &'static str> {
    let client = get_client()?;
    client.flush().await;
    Ok(())
}
