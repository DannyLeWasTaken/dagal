// Contains information regarding the Vulkan instance

use ash::vk;

pub struct Instance {
    pub handle: ash::Instance,
    pub entry: ash::Entry,
}

pub struct InstanceCreateInfo {
    pub required_extensions: Vec<String>,
    pub validation_enabled: bool,
}

impl Instance {
    pub fn new(entry: ash::Entry, required_extensions: Vec<String>) {}
}
