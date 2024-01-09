use std::collections::HashMap;
use ash::vk;
use ash::vk::{TaggedStructure};
use std::sync::Arc;
use abstraction::queue::Queue;
use crate::abstraction::prelude as abstraction;

// Thanks phobos-rs :)
pub struct DeviceInner {
    handle: ash::Device,
    physical_device_features: vk::PhysicalDeviceFeatures2,
}

/// Abstraction for the Vulkan device of Vulkan
#[derive(Clone)]
pub struct Device {
    handle: Arc<DeviceInner>
}

pub struct DeviceBuilder {
    available_extensions: Vec<vk::ExtensionProperties>,
    // (queue_properties, usage times i.e. how many times has it already been used)
    available_queue_families: Vec<(vk::QueueFamilyProperties2, u32)>,

    // (queue_family_index, queueRequest)
    queue_requests: Vec<(u32, QueueRequest)>,
    extension_requests: Vec<String>,
}

impl DeviceBuilder {
    pub fn new(physical_device: &abstraction::PhysicalDevice) -> Self {
        DeviceBuilder {
            available_extensions: physical_device.get_extensions().clone().to_vec(),
            available_queue_families: physical_device.get_queues().clone().iter().map(|queue| (queue, 0)).collect(),
            queue_requests: Vec::new(),
            extension_requests: Vec::new()
        }
    }

    /// This checks and ensures that any queues requested + created will respect
    pub fn request_queue(&mut self, request: QueueRequest) -> Option<u32> {
        // Suitable queues
        for (index, (queue, used)) in self.available_queue_families.iter_mut().enumerate() {
            if queue.queue_family_properties.queue_flags.contains(request.flags) {
                if queue.queue_family_properties.queue_count >= *used + request.count {
                    self.queue_requests.push((index as u32, request));
                    return Some(index as u32)
                }
            }
        }
        return None
    }

    pub fn add_extensions(mut self, extensions: &[String]) -> Self {
        // check if queue is avaliable

        self
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct QueueRequest {
    flags: vk::QueueFlags,
    count: u32,
    unique: bool,
}



impl Device {
    pub fn new(
        instance: ash::Instance,
        physical_device: abstraction::PhysicalDevice,
        requested_queues: &[vk::DeviceQueueCreateInfo],
    ) -> Result<Self, vk::Result> {
        // TODO: allow user to manually pick and choose with queues are used
        let queue_families = physical_device.get_queues();


        let device_ci = vk::DeviceCreateInfo {
            s_type: vk::DeviceCreateInfo::STRUCTURE_TYPE,

            flags: Default::default(),
            queue_create_info_count: requested_queues.len() as u32,
            p_queue_create_infos: requested_queues.as_ptr(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: (),
            p_enabled_features: (),

            ..Default::default()
        };
        let handle = unsafe { instance.create_device(physical_device.get_handle().clone(), &device_ci, None) };
        if handle.is_err() {
            return Err(handle.err().unwrap());
        }
        let handle = handle.unwrap();
        Ok(Self { handle: Arc::new(DeviceInner {
                handle,
                physical_device_features: physical_device.get_features(),

            })
        })
    }

    pub fn handle_as_ref(&self) -> &ash::Device {
        &self.handle.handle
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy_device(None);
        }
    }
}
