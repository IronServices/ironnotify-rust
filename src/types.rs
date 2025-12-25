//! Type definitions for IronNotify SDK.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Severity level for notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SeverityLevel {
    #[default]
    Info,
    Success,
    Warning,
    Error,
    Critical,
}

impl std::fmt::Display for SeverityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Critical => "critical",
        };
        write!(f, "{}", s)
    }
}

/// WebSocket connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionState {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}

impl std::fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Disconnected => "disconnected",
            Self::Connecting => "connecting",
            Self::Connected => "connected",
            Self::Reconnecting => "reconnecting",
        };
        write!(f, "{}", s)
    }
}

/// Action button on a notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationAction {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

impl NotificationAction {
    /// Creates a new action with just a label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            url: None,
            action: None,
            style: Some("default".to_string()),
        }
    }

    /// Creates an action with a URL.
    pub fn with_url(label: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            url: Some(url.into()),
            action: None,
            style: Some("default".to_string()),
        }
    }

    /// Creates an action with an action handler.
    pub fn with_handler(label: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            url: None,
            action: Some(action.into()),
            style: Some("default".to_string()),
        }
    }

    /// Sets the style.
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }
}

/// Payload for creating a notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationPayload {
    pub event_type: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<SeverityLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<NotificationAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplication_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

impl NotificationPayload {
    /// Creates a new notification payload.
    pub fn new(event_type: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            event_type: event_type.into(),
            title: title.into(),
            message: None,
            severity: Some(SeverityLevel::Info),
            metadata: None,
            actions: None,
            user_id: None,
            group_key: None,
            deduplication_key: None,
            expires_at: None,
        }
    }
}

/// A notification received from the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: String,
    pub event_type: String,
    pub title: String,
    #[serde(default)]
    pub message: Option<String>,
    pub severity: SeverityLevel,
    #[serde(default)]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub actions: Option<Vec<NotificationAction>>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub group_key: Option<String>,
    pub read: bool,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
}

/// Result of sending a notification.
#[derive(Debug, Clone)]
pub struct SendResult {
    pub success: bool,
    pub notification_id: Option<String>,
    pub error: Option<String>,
    pub queued: bool,
}

impl SendResult {
    /// Creates a success result.
    pub fn success(notification_id: Option<String>) -> Self {
        Self {
            success: true,
            notification_id,
            error: None,
            queued: false,
        }
    }

    /// Creates a failure result.
    pub fn failure(error: impl Into<String>) -> Self {
        Self {
            success: false,
            notification_id: None,
            error: Some(error.into()),
            queued: false,
        }
    }

    /// Creates a queued result.
    pub fn queued(error: impl Into<String>) -> Self {
        Self {
            success: false,
            notification_id: None,
            error: Some(error.into()),
            queued: true,
        }
    }
}
