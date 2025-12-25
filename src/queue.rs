//! Offline queue for IronNotify SDK.

use crate::types::NotificationPayload;
use parking_lot::Mutex;
use std::fs;
use std::path::PathBuf;

/// Offline queue for storing notifications when offline.
pub struct OfflineQueue {
    max_size: usize,
    debug: bool,
    queue: Mutex<Vec<NotificationPayload>>,
    storage_path: PathBuf,
}

impl OfflineQueue {
    /// Creates a new OfflineQueue.
    pub fn new(max_size: usize, debug: bool) -> Self {
        let storage_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".ironnotify")
            .join("offline_queue.json");

        let queue = Self {
            max_size,
            debug,
            queue: Mutex::new(Vec::new()),
            storage_path,
        };

        queue.load_from_storage();
        queue
    }

    /// Adds a notification to the queue.
    pub fn add(&self, payload: NotificationPayload) {
        let mut queue = self.queue.lock();

        if queue.len() >= self.max_size {
            queue.remove(0);
            if self.debug {
                println!("[IronNotify] Offline queue full, dropping oldest notification");
            }
        }

        if self.debug {
            println!(
                "[IronNotify] Notification queued for later: {}",
                payload.event_type
            );
        }

        queue.push(payload);
        drop(queue);
        self.save_to_storage();
    }

    /// Gets all queued notifications.
    pub fn get_all(&self) -> Vec<NotificationPayload> {
        self.queue.lock().clone()
    }

    /// Removes a notification at the given index.
    pub fn remove(&self, index: usize) {
        let mut queue = self.queue.lock();
        if index < queue.len() {
            queue.remove(index);
            drop(queue);
            self.save_to_storage();
        }
    }

    /// Clears the queue.
    pub fn clear(&self) {
        self.queue.lock().clear();
        self.save_to_storage();
    }

    /// Gets the queue size.
    pub fn size(&self) -> usize {
        self.queue.lock().len()
    }

    /// Checks if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.lock().is_empty()
    }

    fn load_from_storage(&self) {
        if let Ok(data) = fs::read_to_string(&self.storage_path) {
            if let Ok(queue) = serde_json::from_str::<Vec<NotificationPayload>>(&data) {
                *self.queue.lock() = queue;
            }
        }
    }

    fn save_to_storage(&self) {
        if let Some(parent) = self.storage_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string(&*self.queue.lock()) {
            let _ = fs::write(&self.storage_path, json);
        }
    }
}
