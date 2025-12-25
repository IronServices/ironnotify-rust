//! HTTP transport for IronNotify SDK.

use crate::types::{Notification, NotificationPayload, SendResult};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

/// HTTP transport for communicating with the IronNotify API.
pub struct Transport {
    base_url: String,
    api_key: String,
    debug: bool,
    client: Client,
}

#[derive(Deserialize)]
struct SendResponse {
    #[serde(rename = "notificationId")]
    notification_id: Option<String>,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: Option<String>,
}

#[derive(Deserialize)]
struct CountResponse {
    count: i32,
}

impl Transport {
    /// Creates a new Transport.
    pub fn new(base_url: String, api_key: String, timeout: Duration, debug: bool) -> Self {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url,
            api_key,
            debug,
            client,
        }
    }

    /// Sends a notification payload.
    pub async fn send(&self, payload: &NotificationPayload) -> SendResult {
        if self.debug {
            println!("[IronNotify] Sending notification: {}", payload.event_type);
        }

        let result = self
            .client
            .post(format!("{}/api/v1/notify", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(payload)
            .send()
            .await;

        match result {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(data) = response.json::<SendResponse>().await {
                        SendResult::success(data.notification_id)
                    } else {
                        SendResult::success(None)
                    }
                } else {
                    let status = response.status();
                    if let Ok(error) = response.json::<ErrorResponse>().await {
                        SendResult::failure(
                            error
                                .error
                                .unwrap_or_else(|| format!("HTTP {}", status)),
                        )
                    } else {
                        SendResult::failure(format!("HTTP {}", status))
                    }
                }
            }
            Err(e) => SendResult::failure(e.to_string()),
        }
    }

    /// Gets notifications.
    pub async fn get_notifications(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        unread_only: bool,
    ) -> Result<Vec<Notification>, String> {
        let mut url = format!("{}/api/v1/notifications", self.base_url);
        let mut params = Vec::new();

        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            params.push(format!("offset={}", o));
        }
        if unread_only {
            params.push("unread_only=true".to_string());
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let result = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        match result {
            Ok(response) => {
                if response.status().is_success() {
                    response
                        .json()
                        .await
                        .map_err(|e| e.to_string())
                } else {
                    Err(format!("HTTP {}", response.status()))
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    /// Gets the unread notification count.
    pub async fn get_unread_count(&self) -> Result<i32, String> {
        let result = self
            .client
            .get(format!("{}/api/v1/notifications/unread-count", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        match result {
            Ok(response) => {
                if response.status().is_success() {
                    let data: CountResponse = response.json().await.map_err(|e| e.to_string())?;
                    Ok(data.count)
                } else {
                    Err(format!("HTTP {}", response.status()))
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    /// Marks a notification as read.
    pub async fn mark_as_read(&self, notification_id: &str) -> Result<bool, String> {
        let result = self
            .client
            .post(format!(
                "{}/api/v1/notifications/{}/read",
                self.base_url, notification_id
            ))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        match result {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Marks all notifications as read.
    pub async fn mark_all_as_read(&self) -> Result<bool, String> {
        let result = self
            .client
            .post(format!("{}/api/v1/notifications/read-all", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        match result {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Checks if the API is reachable.
    pub async fn is_online(&self) -> bool {
        if let Ok(response) = self
            .client
            .get(format!("{}/health", self.base_url))
            .send()
            .await
        {
            response.status().is_success()
        } else {
            false
        }
    }
}
