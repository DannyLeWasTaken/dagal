// Contains information regarding the Vulkan instance

use std::sync::Arc;
use ash::vk;

struct InstanceInner {
    handle: ash::Instance,
    entry: ash::Entry,
}

#[derive(Clone)]
pub struct Instance {
    handle: Arc<InstanceInner>,
}

pub struct InstanceCreateInfo {
    pub required_extensions: Vec<String>,
    pub validation_enabled: bool,
}

impl Instance {
    pub fn new(handle: ash::Instance, entry: ash::Entry) -> Self {
        Instance {
            handle: Arc::new(
                InstanceInner { handle, entry }
            )
        }
    }

    pub fn get_vk_instance(&self) -> &ash::Instance {
        &self.handle.handle
    }

    pub fn get_vk_entry(&self) -> &ash::Entry {
        &self.handle.entry
    }
}

impl Drop for InstanceInner {
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy_instance(None);
        }
    }
}