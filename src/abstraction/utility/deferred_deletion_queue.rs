//! Almost all lifetimes need to be deleted in a deferred manner.
//!
//! While a resource may be "unused" on the CPU (i.e. reference count = 0), it may
//! still be use on the GPU such as being used in a command buffer that is in flight.
//!
//! Deferred delete allows us to defer this deletion process while still allowing us
//! to turn their CPU representation of the resource into a "husk" making it unusable
//! from the CPU.
//! This pattern is implemented by simply creating a private facing "Inner" struct as dropping
//! the public facing struct will make the Inner inaccessible.
use std::sync::{Arc, Mutex};

struct DeferredDeletionItem<T> {
    item: T,
    /// Cycle # which the deferred item is deleted.
    end_cycle: u64,
}

/// Any struct that implements [DeferredDeletable] can be pushed into a deferred deletion
/// queue.
pub trait DeferredDeletable {
    /// Maximum number of cycles a struct should last for when submitted
    const MAX_CYCLES: u64;
}

/// This is a deferred deletion queue
pub struct DeferredDeletionQueue<T> {
    /// Represents the # of cycles that has passed
    cycle: u64,
    /// Represents the items in queue
    items: Arc<Mutex<Vec<DeferredDeletionItem<T>>>>,
}

impl<T> DeferredDeletionQueue<T> {
    pub fn new() -> Self {
        Self {
            cycle: 0,
            items: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Queues item for deletion
    pub fn enqueue_deletion(&mut self, resource: T, deletion_function: Option<Box<dyn FnOnce()>>) {
        self.items.lock().unwrap().push(DeferredDeletionItem {
            item: resource,
            end_cycle: T::MAX_CYCLES,
        });
    }

    /// Scans the entire queue to finds items in queue that have reached their
    /// end cycle
    pub fn delete_expired_items(&mut self) {
        self.items.lock().unwrap().retain(|item| {
            if item.end_cycle >= self.cycle {
                // Expired, run delete function and drop the item from queue
                false
            } else {
                // Keep all items in queue that have not expired yet
                true
            }
        })
    }

    /// Step forward by one cycle
    pub fn step(&mut self) {
        self.cycle += 1;
    }
}
