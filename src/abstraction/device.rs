use crate::abstraction::prelude as abstraction;
use ash::vk;
use ash::vk::TaggedStructure;
use std::ffi::{c_char, CString};
use std::ptr;
use std::sync::Arc;

/// Provides information on queue family on the device
pub struct DeviceFamilyQueue {
    flags: vk::QueueFlags,
    index: u32,
}

// Thanks phobos-rs :)
/// Represents the underlying implementation.
/// Primarily done to force all instances of [Device] to reference count the [DeviceInner] to allow for easier
/// lifetime management.
pub struct DeviceInner {
    handle: ash::Device,
    /// Features of the underlying [vk::PhysicalDevice]
    physical_device_features: vk::PhysicalDeviceFeatures2,
    /// All queue families
    queue_families: Vec<DeviceFamilyQueue>,
}

impl Drop for DeviceInner {
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy_device(None);
        }
    }
}

/// Abstraction for the Vulkan device of Vulkan
#[derive(Clone)]
pub struct Device {
    handle: Arc<DeviceInner>,
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
        device_features: vk::PhysicalDeviceFeatures2,
    ) -> Result<Self, vk::Result> {
        // TODO: allow user to manually pick and choose with queues are used
        let gpu_requirements = physical_device.get_gpu_requirements();
        // keeps tracks of all queue requirements that have been met and as such can be ignored by future iterations
        let queue_requirements_met: Vec<bool> = vec![false; gpu_requirements.queues.len()];
        let queue_cis: Vec<vk::DeviceQueueCreateInfo> = physical_device
            .get_queue_families()
            .iter()
            .enumerate()
            .filter_map(|(queue_family_index, queue_family)| {
                let mut queue_family_used_slots: u32 = 0; // Track the # of queue slots used already

                // queue families may have multiple queues which are applicable to them
                let queue_count: u32 = gpu_requirements
                    .queues
                    .iter()
                    .enumerate()
                    .map(|(queue_requirements_index, queue_requirements)| {
                        // If we were to create the queue, it would overflow so skip
                        // OR this queue is already being used
                        if *queue_requirements_met
                            .get(queue_requirements_index)
                            .unwrap()
                            == true
                            || queue_family_used_slots + queue_requirements.count
                                > queue_family.queue_family_properties.queue_count
                        {
                            return 0u32;
                        }
                        if (queue_requirements.dedicated
                            && queue_requirements.queue_flags
                                == queue_family.queue_family_properties.queue_flags)
                            && (!queue_requirements.dedicated
                                && queue_family
                                    .queue_family_properties
                                    .queue_flags
                                    .contains(queue_requirements.queue_flags))
                        {
                            queue_family_used_slots += queue_requirements.count;
                            return queue_requirements.count;
                        }
                        0u32
                    })
                    .sum();
                // If queue_count is zero, we don't need to make a queue at all
                if queue_count == 0 {
                    return None;
                } else if queue_requirements_met.iter().all(|x| *x) == false {
                    // queue requirements were not met and as such, we can ignore this device
                    return None;
                }
                Some(vk::DeviceQueueCreateInfo {
                    s_type: vk::DeviceQueueCreateInfo::STRUCTURE_TYPE,
                    flags: vk::DeviceQueueCreateFlags::empty(),
                    queue_family_index: queue_family_index as u32,
                    queue_count,
                    p_queue_priorities: &1.0, // TODO: get proper queue priorities
                    ..Default::default()
                })
            })
            .collect();
        let c_ptrs: Vec<*const c_char> = gpu_requirements
            .extensions
            .iter()
            .map(|ext| CString::new(ext.as_str()).unwrap().into_raw() as *const c_char)
            .collect();
        //

        let device_ci = vk::DeviceCreateInfo {
            s_type: vk::DeviceCreateInfo::STRUCTURE_TYPE,
            p_next: abstraction::utility::p_next(&device_features),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: queue_cis.len() as u32,
            p_queue_create_infos: queue_cis.as_ptr(),
            enabled_extension_count: gpu_requirements.extensions.len() as u32,
            pp_enabled_extension_names: c_ptrs.as_ptr(),
            p_enabled_features: ptr::null(),

            ..Default::default()
        };
        let handle = unsafe {
            instance.create_device(physical_device.get_handle().clone(), &device_ci, None)
        };
        if handle.is_err() {
            return Err(handle.err().unwrap());
        }
        let handle = handle.unwrap();
        Ok(Self {
            handle: Arc::new(DeviceInner {
                handle,
                physical_device_features: physical_device.get_features(),
                queue_families: vec![],
            }),
        })
    }

    pub fn handle_as_ref(&self) -> &ash::Device {
        &self.handle.handle
    }
}
