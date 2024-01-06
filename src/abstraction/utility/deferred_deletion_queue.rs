//! Almost all lifetimes need to be deleted in a deferred manner.
//!
//! While a resource may be "unused" on the CPU (i.e. reference count = 0), it may
//! still be use on the GPU such as being used in a command buffer that is in flight.
//!
//! Deferred delete allows us to defer this deletion process while still allowing us
//! to turn their CPU representation of the resource into a "husk" making it unusable
//! from the CPU.
use std::sync::{Arc, Mutex, Weak};

/// Defines the two state any [Huskable] struct can be in
enum HuskState {
	Valid,
	Husk,
}

/// This trait when implemented allows any struct to be turned into a "husk" where
/// it is not longer usable by the CPU
trait Huskable {
	/// Get current husk state of the struct
	fn get_husk_state(&self) -> HuskState;
	/// Turn the struct into a husk and hence, make it invalid to use
	fn make_husk(&mut self);
}

struct DeferredDeletionItem<T> {
	/// Cycle # which the deferred item is deleted.
	end_cycle: u64,
	/// Executes upon deletion, but may be optional as some structs may simply only
	/// rely on the drop functionality
	deletion_function: Option<Box<dyn FnOnce()>>,
	/// Underlying resource represented
	resource: Box<T>,
}

/// This is a deferred deletion queue
pub struct DeferredDeletionQueue {
	/// Represents the # of cycles that has passed
	cycle: u64,
	/// Represents the items in queue
	items: Arc<Mutex<Vec<DeferredDeletionItem<dyn Drop>>>>,
}


impl DeferredDeletionQueue {
	pub fn new() -> Self {
		Self {
			cycle: 0,
			items: Arc::new(Mutex::new(Vec::new()))
		}
	}

	/// Queues item for deletion
	pub fn enqueue_deletion(&mut self, local_end_cycle: u64, ) {
		self.items.get_mut().unwrap()
			.push()
	}

	/// Scans the entire queue to finds items in queue that have reached their
	/// end cycle
	pub fn delete_expired_items(&mut self) {
		self.items.get_mut().unwrap().retain(|item| {
			if item.end_cycle >= self.cycle {
				// Expired, run delete function and drop the item from queue
				item.deletion_function();
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

impl DeferredDeletionQueue {

}