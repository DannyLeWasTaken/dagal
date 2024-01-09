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
pub(crate) const MINIMUM_VIABLE_REQUIREMENTS: PhysicalDeviceRequirements =
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
    };

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
pub fn select_suitable_physical_device(
    instance: abstraction::Instance,
    gpu_requirements: Option<PhysicalDeviceRequirements>,
) -> Vec<super::PhysicalDevice> {
    // this will be narrowed down
    let suitable_physical_devices: Vec<Option<super::PhysicalDevice>> = unsafe {
        instance
            .handle
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
    }
    suitable_physical_devices
}
