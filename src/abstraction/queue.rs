use ash::vk;

/// An abstraction for the queue object in Vulkan
pub struct Queue {
    handle: vk::Queue,
    queue_flags: vk::QueueFlags,
    index: u32,
}

impl Queue {
    pub fn new(handle: vk::Queue) {}
}

impl Into<u32> for Queue {
    /// Retrives the queue index for the related physical device
    fn into(self) -> u32 {
        self.index
    }
}
