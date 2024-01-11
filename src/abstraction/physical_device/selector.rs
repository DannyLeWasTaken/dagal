use crate::abstraction::prelude as abstraction;
use ash::vk;
/// Indicate queue requirements
#[derive(Clone, PartialOrd, PartialEq)]
pub struct QueueRequirements {
    /// Flags of the queue in question
    pub queue_flags: vk::QueueFlags,

    /// Whether or not the queue should be dedicated
    pub dedicated: bool,

    /// How much many queues exist in the family
    pub count: u32,
}

/// Describes the requirements for to select a physical device
#[derive(Clone, Default, PartialOrd, PartialEq)]
pub struct PhysicalDeviceRequirements {
    pub extensions: Vec<String>,
    pub queues: Vec<QueueRequirements>,
}

// These are minimum requirements we expect any GPU should have.
pub fn get_minimum_viable_requirements() -> PhysicalDeviceRequirements {
    PhysicalDeviceRequirements {
        extensions: vec![
            ash::extensions::khr::BufferDeviceAddress::name(),
            ash::extensions::khr::DynamicRendering::name(),
            ash::extensions::khr::Synchronization2::name(),
        ]
            .iter()
            .map(|ext_name| ext_name.to_string_lossy().into_owned())
            .collect(),
        queues: Vec::new(),
    }
}

pub struct PhysicalDeviceSelector {
    instance: abstraction::Instance,
}

/// Returns a [Vec] containing all [super::PhysicalDevice] that satisfy [PhysicalDeviceRequirements] given.
/// # Queue safety
/// While we do filter physical devices ensuring that:
///
/// 1) The queue family exists
///
/// 2) Enough of said queue can be
/// allocated for each [QueueRequirements]
///
/// (2) is **not checked globally**. That is to say: if the total # of queues
/// used by all [QueueRequirements] exceeds the queue count in the queue family, we don't detect that behavior.
///
/// # Usage
pub fn select_suitable_physical_device(
    instance: abstraction::Instance,
    gpu_requirements: Option<PhysicalDeviceRequirements>,
) -> Vec<super::PhysicalDevice> {
    // this will be narrowed down
    let suitable_physical_devices: Vec<Option<super::PhysicalDevice>> = unsafe {
        instance
            .get_vk_instance()
            .enumerate_physical_devices()
            .expect("Failed to enumerate through physical devices!")
    }
    .iter()
    .map(|pd| super::PhysicalDevice::new(instance.clone(), pd.clone(), gpu_requirements.clone()))
    .collect();
    let suitable_physical_devices: Vec<super::PhysicalDevice> = suitable_physical_devices
        .into_iter()
        .filter_map(|x| x)
        .collect();
    // Check for family queues
    if let Some(gpu_requirements) = gpu_requirements {
        let suitable_physical_devices: Vec<super::PhysicalDevice> =
            suitable_physical_devices
                .into_iter()
                .filter_map(|physical_device| {
                    let queue_families = physical_device.get_queue_families();
                    let queue_requirements_met = gpu_requirements.queues.iter().all(
                        |queue_requirements: &QueueRequirements| {
                            let queue_possible: bool = queue_families.iter().any(|queue_family| {
                                if ((queue_requirements.dedicated
                                    && queue_requirements.queue_flags
                                        == queue_family.queue_family_properties.queue_flags)
                                    || (!queue_requirements.dedicated
                                        && queue_family
                                            .queue_family_properties
                                            .queue_flags
                                            .contains(queue_requirements.queue_flags)))
                                    && queue_family.queue_family_properties.queue_count
                                        >= queue_requirements.count
                                {
                                    return true;
                                }
                                false
                            });
                            queue_possible
                        },
                    );
                    if queue_requirements_met {
                        Some(physical_device)
                    } else {
                        None
                    }
                })
                .collect();
        return suitable_physical_devices;
    }
    suitable_physical_devices
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module
use std::ffi::c_char;
    use std::ptr;
    use ash;
    use ash::vk;
    use ash::vk::TaggedStructure;
    use crate::abstraction::prelude as abstraction;

    #[test]
    /// Tests the physical device selection and creation of the device subsequently
    fn test_physical_device_selection() {
        let validation_layer: [&'static str; 1] = ["VK_LAYER_KHRONOS_validation"];
        // Get ash instance
        let application_name = "Test app";
        let engine_name = "Test engine";
        let ash_entry = ash::Entry::linked();
        let app_info = vk::ApplicationInfo {
            s_type: vk::ApplicationInfo::STRUCTURE_TYPE,
            p_next: ptr::null(),
            p_application_name: &application_name as *const _ as *const c_char,
            application_version: 0,
            p_engine_name: &engine_name as *const _ as *const c_char,
            engine_version: 0,
            api_version: vk::API_VERSION_1_3,
        };
        let ash_instance = unsafe { ash_entry.create_instance(&vk::InstanceCreateInfo {
            s_type: vk::InstanceCreateInfo::STRUCTURE_TYPE,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: ptr::null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: ptr::null(),
        }, None).unwrap() };
        let instance = abstraction::Instance::new(ash_instance, ash_entry,);
        let select_physical_device = abstraction::select_suitable_physical_device(instance.clone(), None).pop().unwrap();
        let features = select_physical_device.get_features();
        let device = abstraction::Device::new(&instance, select_physical_device).unwrap();

        // Clean up
        drop(device);
        drop(instance);
    }
}