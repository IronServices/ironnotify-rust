# IronNotify SDK for Rust

Event notifications and alerts SDK for Rust applications. Send notifications, receive real-time updates, and manage notification state.

[![Crates.io](https://img.shields.io/crates/v/ironnotify.svg)](https://crates.io/crates/ironnotify)
[![Documentation](https://docs.rs/ironnotify/badge.svg)](https://docs.rs/ironnotify)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ironnotify = "0.1"
tokio = { version = "1.0", features = ["rt-multi-thread"] }
```

## Quick Start

### Send a Simple Notification

```rust
use ironnotify::{NotifyClient, NotifyOptions};

#[tokio::main]
async fn main() {
    // Initialize
    let client = NotifyClient::new(NotifyOptions::new("ak_live_xxxxx"))
        .expect("Failed to create client");

    // Send a simple notification
    let result = client.notify("order.created", "New Order Received").await;

    if result.success {
        println!("Notification sent: {:?}", result.notification_id);
    }
}
```

### Fluent Event Builder

```rust
use ironnotify::{NotifyClient, NotifyOptions, SeverityLevel, NotificationAction};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let client = NotifyClient::new(NotifyOptions::new("ak_live_xxxxx"))
        .expect("Failed to create client");

    // Build complex notifications with the fluent API
    let result = client.event("payment.failed")
        .with_title("Payment Failed")
        .with_message("Payment could not be processed")
        .with_severity(SeverityLevel::Error)
        .with_metadata("order_id", "1234")
        .with_metadata("reason", "Card declined")
        .with_url_action("Retry Payment", "/orders/1234/retry")
        .with_action(NotificationAction::with_handler("Contact Support", "open_support"))
        .for_user("user-123")
        .with_deduplication_key("payment-failed-1234")
        .expires_in_std(Duration::from_secs(86400))
        .send()
        .await;

    if result.queued {
        println!("Notification queued for later");
    }
}
```

### Using the Global Client

```rust
use ironnotify::{self, SeverityLevel};

#[tokio::main]
async fn main() {
    // Initialize global client
    ironnotify::init("ak_live_xxxxx").expect("Failed to init");

    // Send notification
    let result = ironnotify::notify("event.type", "Title")
        .await
        .expect("Failed to send");

    // Use event builder
    let result = ironnotify::event("event.type")
        .expect("Client not initialized")
        .with_title("Title")
        .send()
        .await;

    // Flush offline queue
    ironnotify::flush().await.ok();
}
```

## Configuration

```rust
use ironnotify::{NotifyClient, NotifyOptions};
use std::time::Duration;

let client = NotifyClient::new(
    NotifyOptions::builder()
        .api_key("ak_live_xxxxx")
        .api_base_url("https://api.ironnotify.com")
        .ws_url("wss://ws.ironnotify.com")
        .debug(false)
        .enable_offline_queue(true)
        .max_offline_queue_size(100)
        .auto_reconnect(true)
        .max_reconnect_attempts(5)
        .reconnect_delay(Duration::from_secs(1))
        .http_timeout(Duration::from_secs(30))
        .build()
        .expect("Invalid options")
).expect("Failed to create client");
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `api_key` | String | required | Your API key (ak_live_xxx or ak_test_xxx) |
| `api_base_url` | String | https://api.ironnotify.com | API base URL |
| `ws_url` | String | wss://ws.ironnotify.com | WebSocket URL |
| `debug` | bool | false | Enable debug logging |
| `enable_offline_queue` | bool | true | Queue notifications when offline |
| `max_offline_queue_size` | usize | 100 | Max offline queue size |
| `auto_reconnect` | bool | true | Auto-reconnect WebSocket |
| `max_reconnect_attempts` | u32 | 5 | Max reconnection attempts |
| `reconnect_delay` | Duration | 1s | Base reconnection delay |
| `http_timeout` | Duration | 30s | HTTP request timeout |

## Severity Levels

```rust
use ironnotify::SeverityLevel;

SeverityLevel::Info     // "info"
SeverityLevel::Success  // "success"
SeverityLevel::Warning  // "warning"
SeverityLevel::Error    // "error"
SeverityLevel::Critical // "critical"
```

## Actions

```rust
use ironnotify::NotificationAction;

// Action with URL
client.event("order.shipped")
    .with_title("Order Shipped")
    .with_url_action("Track Package", "https://tracking.example.com/123")
    .send()
    .await;

// Action with handler
client.event("order.shipped")
    .with_title("Order Shipped")
    .with_handler_action("View Order", "view_order")
    .send()
    .await;

// Custom action with style
client.event("order.shipped")
    .with_title("Order Shipped")
    .with_action(
        NotificationAction::with_url("Track Package", "https://tracking.example.com/123")
            .style("primary")
    )
    .send()
    .await;
```

## Deduplication

Prevent duplicate notifications:

```rust
client.event("reminder")
    .with_title("Daily Reminder")
    .with_deduplication_key("daily-reminder-2024-01-15")
    .send()
    .await;
```

## Grouping

Group related notifications:

```rust
client.event("comment.new")
    .with_title("New Comment")
    .with_group_key("post-123-comments")
    .send()
    .await;
```

## Expiration

```rust
use chrono::{Duration, Utc};
use std::time::Duration as StdDuration;

// Expires in 1 hour (using chrono Duration)
client.event("flash_sale")
    .with_title("Flash Sale!")
    .expires_in(Duration::hours(1))
    .send()
    .await;

// Expires in 1 hour (using std Duration)
client.event("flash_sale")
    .with_title("Flash Sale!")
    .expires_in_std(StdDuration::from_secs(3600))
    .send()
    .await;

// Expires at specific time
client.event("event_reminder")
    .with_title("Event Tomorrow")
    .expires_at(Utc::now() + Duration::days(1))
    .send()
    .await;
```

## Managing Notifications

### Get Notifications

```rust
// Get all notifications
let notifications = client.get_notifications(None, None, false).await?;

// With options
let unread = client.get_notifications(Some(10), Some(0), true).await?;
```

### Mark as Read

```rust
// Mark single notification
client.mark_as_read("notification-id").await?;

// Mark all as read
client.mark_all_as_read().await?;
```

### Get Unread Count

```rust
let count = client.get_unread_count().await?;
println!("You have {} unread notifications", count);
```

## Real-Time Notifications

```rust
let client = NotifyClient::new(NotifyOptions::new("ak_live_xxxxx"))
    .expect("Failed to create client");

client.connect();
client.subscribe_to_user("user-123");
client.subscribe_to_app();

// Check connection state
let state = client.connection_state();
println!("Connection state: {}", state);
```

## Offline Support

Notifications are automatically queued when offline:

```rust
// This will be queued if offline
client.notify("event", "Title").await;

// Manually flush the queue
client.flush().await;
```

## Thread Safety

The client is thread-safe and can be shared across threads using `Arc<NotifyClient>`.

## Requirements

- Rust 1.70+
- Tokio runtime

## Links

- [Documentation](https://www.ironnotify.com/docs)
- [Dashboard](https://www.ironnotify.com)

## License

MIT License - see [LICENSE](LICENSE) for details.
