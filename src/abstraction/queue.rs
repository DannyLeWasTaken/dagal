use crate::abstraction::prelude as abstraction;
use ash::vk;

/// An abstraction for [vk::Queue]
pub struct Queue {
    handle: vk::Queue,
    family_index: u32,
    device: abstraction::Device,
}

impl Queue {
    pub fn new(handle: vk::Queue) {}
}

impl Into<u32> for Queue {
    /// Retrieves the queue index for the related physical device
    fn into(self) -> u32 {
        self.family_index
    }
}
