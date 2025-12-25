//! Event builder for IronNotify SDK.

use crate::client::NotifyClient;
use crate::types::{NotificationAction, NotificationPayload, SendResult, SeverityLevel};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;

/// Builder for creating notifications with a fluent API.
pub struct EventBuilder {
    client: Arc<NotifyClient>,
    event_type: String,
    title: Option<String>,
    message: Option<String>,
    severity: SeverityLevel,
    metadata: HashMap<String, serde_json::Value>,
    actions: Vec<NotificationAction>,
    user_id: Option<String>,
    group_key: Option<String>,
    deduplication_key: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

impl EventBuilder {
    /// Creates a new EventBuilder.
    pub(crate) fn new(client: Arc<NotifyClient>, event_type: impl Into<String>) -> Self {
        Self {
            client,
            event_type: event_type.into(),
            title: None,
            message: None,
            severity: SeverityLevel::Info,
            metadata: HashMap::new(),
            actions: Vec::new(),
            user_id: None,
            group_key: None,
            deduplication_key: None,
            expires_at: None,
        }
    }

    /// Sets the notification title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the notification message.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Sets the severity level.
    pub fn with_severity(mut self, severity: SeverityLevel) -> Self {
        self.severity = severity;
        self
    }

    /// Adds a metadata entry.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Adds an action button.
    pub fn with_action(mut self, action: NotificationAction) -> Self {
        self.actions.push(action);
        self
    }

    /// Adds an action button with a URL.
    pub fn with_url_action(mut self, label: impl Into<String>, url: impl Into<String>) -> Self {
        self.actions.push(NotificationAction::with_url(label, url));
        self
    }

    /// Adds an action button with a handler.
    pub fn with_handler_action(mut self, label: impl Into<String>, handler: impl Into<String>) -> Self {
        self.actions.push(NotificationAction::with_handler(label, handler));
        self
    }

    /// Sets the target user ID.
    pub fn for_user(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Sets the group key for grouping related notifications.
    pub fn with_group_key(mut self, group_key: impl Into<String>) -> Self {
        self.group_key = Some(group_key.into());
        self
    }

    /// Sets the deduplication key.
    pub fn with_deduplication_key(mut self, key: impl Into<String>) -> Self {
        self.deduplication_key = Some(key.into());
        self
    }

    /// Sets the expiration time from now.
    pub fn expires_in(mut self, duration: Duration) -> Self {
        self.expires_at = Some(Utc::now() + duration);
        self
    }

    /// Sets the expiration time from now (std Duration).
    pub fn expires_in_std(mut self, duration: std::time::Duration) -> Self {
        self.expires_at = Some(Utc::now() + Duration::from_std(duration).unwrap_or(Duration::zero()));
        self
    }

    /// Sets the expiration time.
    pub fn expires_at(mut self, time: DateTime<Utc>) -> Self {
        self.expires_at = Some(time);
        self
    }

    /// Builds the notification payload.
    pub fn build(self) -> Result<NotificationPayload, &'static str> {
        let title = self.title.ok_or("Notification title is required")?;

        Ok(NotificationPayload {
            event_type: self.event_type,
            title,
            message: self.message,
            severity: Some(self.severity),
            metadata: if self.metadata.is_empty() {
                None
            } else {
                Some(self.metadata)
            },
            actions: if self.actions.is_empty() {
                None
            } else {
                Some(self.actions)
            },
            user_id: self.user_id,
            group_key: self.group_key,
            deduplication_key: self.deduplication_key,
            expires_at: self.expires_at,
        })
    }

    /// Sends the notification.
    pub async fn send(self) -> SendResult {
        match self.build() {
            Ok(payload) => self.client.send_payload(&payload).await,
            Err(e) => SendResult::failure(e),
        }
    }
}
