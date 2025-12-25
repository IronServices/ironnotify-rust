//! Configuration options for IronNotify SDK.

use std::time::Duration;

/// Configuration options for the IronNotify client.
#[derive(Debug, Clone)]
pub struct NotifyOptions {
    /// API key for authentication (required).
    /// Format: ak_live_xxx or ak_test_xxx
    pub api_key: String,
    /// Base URL for the IronNotify API.
    pub api_base_url: String,
    /// WebSocket URL for real-time notifications.
    pub ws_url: String,
    /// Enable debug logging.
    pub debug: bool,
    /// Enable offline notification queuing.
    pub enable_offline_queue: bool,
    /// Maximum number of notifications to queue offline.
    pub max_offline_queue_size: usize,
    /// Enable automatic WebSocket reconnection.
    pub auto_reconnect: bool,
    /// Maximum number of reconnection attempts.
    pub max_reconnect_attempts: u32,
    /// Base delay between reconnection attempts.
    pub reconnect_delay: Duration,
    /// HTTP request timeout.
    pub http_timeout: Duration,
}

impl NotifyOptions {
    /// Creates new options with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            ..Default::default()
        }
    }

    /// Creates a builder for NotifyOptions.
    pub fn builder() -> NotifyOptionsBuilder {
        NotifyOptionsBuilder::default()
    }
}

impl Default for NotifyOptions {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            api_base_url: "https://api.ironnotify.com".to_string(),
            ws_url: "wss://ws.ironnotify.com".to_string(),
            debug: false,
            enable_offline_queue: true,
            max_offline_queue_size: 100,
            auto_reconnect: true,
            max_reconnect_attempts: 5,
            reconnect_delay: Duration::from_secs(1),
            http_timeout: Duration::from_secs(30),
        }
    }
}

/// Builder for NotifyOptions.
#[derive(Debug, Default)]
pub struct NotifyOptionsBuilder {
    options: NotifyOptions,
}

impl NotifyOptionsBuilder {
    /// Sets the API key.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.options.api_key = api_key.into();
        self
    }

    /// Sets the API base URL.
    pub fn api_base_url(mut self, url: impl Into<String>) -> Self {
        self.options.api_base_url = url.into();
        self
    }

    /// Sets the WebSocket URL.
    pub fn ws_url(mut self, url: impl Into<String>) -> Self {
        self.options.ws_url = url.into();
        self
    }

    /// Enables or disables debug mode.
    pub fn debug(mut self, debug: bool) -> Self {
        self.options.debug = debug;
        self
    }

    /// Enables or disables the offline queue.
    pub fn enable_offline_queue(mut self, enable: bool) -> Self {
        self.options.enable_offline_queue = enable;
        self
    }

    /// Sets the maximum offline queue size.
    pub fn max_offline_queue_size(mut self, size: usize) -> Self {
        self.options.max_offline_queue_size = size;
        self
    }

    /// Enables or disables auto-reconnect.
    pub fn auto_reconnect(mut self, enable: bool) -> Self {
        self.options.auto_reconnect = enable;
        self
    }

    /// Sets the maximum reconnect attempts.
    pub fn max_reconnect_attempts(mut self, attempts: u32) -> Self {
        self.options.max_reconnect_attempts = attempts;
        self
    }

    /// Sets the reconnect delay.
    pub fn reconnect_delay(mut self, delay: Duration) -> Self {
        self.options.reconnect_delay = delay;
        self
    }

    /// Sets the HTTP timeout.
    pub fn http_timeout(mut self, timeout: Duration) -> Self {
        self.options.http_timeout = timeout;
        self
    }

    /// Builds the NotifyOptions.
    pub fn build(self) -> Result<NotifyOptions, &'static str> {
        if self.options.api_key.is_empty() {
            return Err("API key is required");
        }
        Ok(self.options)
    }
}
