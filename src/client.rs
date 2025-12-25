//! Main client for IronNotify SDK.

use crate::builder::EventBuilder;
use crate::config::NotifyOptions;
use crate::queue::OfflineQueue;
use crate::transport::Transport;
use crate::types::{ConnectionState, Notification, NotificationPayload, SendResult, SeverityLevel};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

/// IronNotify client for sending and receiving notifications.
pub struct NotifyClient {
    options: NotifyOptions,
    transport: Transport,
    queue: Option<OfflineQueue>,
    is_online: RwLock<bool>,
    connection_state: RwLock<ConnectionState>,
}

impl NotifyClient {
    /// Creates a new NotifyClient.
    pub fn new(options: NotifyOptions) -> Result<Arc<Self>, &'static str> {
        if options.api_key.is_empty() {
            return Err("API key is required");
        }

        let transport = Transport::new(
            options.api_base_url.clone(),
            options.api_key.clone(),
            options.http_timeout,
            options.debug,
        );

        let queue = if options.enable_offline_queue {
            Some(OfflineQueue::new(options.max_offline_queue_size, options.debug))
        } else {
            None
        };

        if options.debug {
            println!("[IronNotify] Client initialized");
        }

        Ok(Arc::new(Self {
            options,
            transport,
            queue,
            is_online: RwLock::new(true),
            connection_state: RwLock::new(ConnectionState::Disconnected),
        }))
    }

    /// Sends a simple notification.
    pub async fn notify(
        self: &Arc<Self>,
        event_type: impl Into<String>,
        title: impl Into<String>,
    ) -> SendResult {
        let payload = NotificationPayload::new(event_type, title);
        self.send_payload(&payload).await
    }

    /// Sends a notification with options.
    pub async fn notify_with_options(
        self: &Arc<Self>,
        event_type: impl Into<String>,
        title: impl Into<String>,
        message: Option<String>,
        severity: Option<SeverityLevel>,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> SendResult {
        let mut payload = NotificationPayload::new(event_type, title);
        payload.message = message;
        payload.severity = severity.or(Some(SeverityLevel::Info));
        payload.metadata = metadata;
        self.send_payload(&payload).await
    }

    /// Creates an event builder.
    pub fn event(self: &Arc<Self>, event_type: impl Into<String>) -> EventBuilder {
        EventBuilder::new(Arc::clone(self), event_type)
    }

    /// Sends a notification payload.
    pub async fn send_payload(self: &Arc<Self>, payload: &NotificationPayload) -> SendResult {
        let result = self.transport.send(payload).await;

        if !result.success {
            if let Some(ref queue) = self.queue {
                queue.add(payload.clone());
                *self.is_online.write() = false;
                return SendResult::queued(result.error.unwrap_or_default());
            }
        }

        result
    }

    /// Gets notifications.
    pub async fn get_notifications(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        unread_only: bool,
    ) -> Result<Vec<Notification>, String> {
        self.transport.get_notifications(limit, offset, unread_only).await
    }

    /// Gets the unread notification count.
    pub async fn get_unread_count(&self) -> Result<i32, String> {
        self.transport.get_unread_count().await
    }

    /// Marks a notification as read.
    pub async fn mark_as_read(&self, notification_id: &str) -> Result<bool, String> {
        self.transport.mark_as_read(notification_id).await
    }

    /// Marks all notifications as read.
    pub async fn mark_all_as_read(&self) -> Result<bool, String> {
        self.transport.mark_all_as_read().await
    }

    /// Gets the current connection state.
    pub fn connection_state(&self) -> ConnectionState {
        *self.connection_state.read()
    }

    /// Connects to real-time notifications.
    pub fn connect(&self) {
        *self.connection_state.write() = ConnectionState::Connected;
        if self.options.debug {
            println!("[IronNotify] Connected (WebSocket not implemented)");
        }
    }

    /// Disconnects from real-time notifications.
    pub fn disconnect(&self) {
        *self.connection_state.write() = ConnectionState::Disconnected;
    }

    /// Subscribes to a user's notifications.
    pub fn subscribe_to_user(&self, user_id: &str) {
        if self.options.debug {
            println!("[IronNotify] Subscribed to user: {}", user_id);
        }
    }

    /// Subscribes to app-wide notifications.
    pub fn subscribe_to_app(&self) {
        if self.options.debug {
            println!("[IronNotify] Subscribed to app notifications");
        }
    }

    /// Flushes the offline queue.
    pub async fn flush(&self) {
        if let Some(ref queue) = self.queue {
            if queue.is_empty() {
                return;
            }

            if !self.transport.is_online().await {
                return;
            }

            *self.is_online.write() = true;
            let notifications = queue.get_all();

            for (i, payload) in notifications.iter().enumerate().rev() {
                let result = self.transport.send(payload).await;
                if result.success {
                    queue.remove(i);
                } else {
                    break;
                }
            }
        }
    }
}
