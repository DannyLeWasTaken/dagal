use ash::vk;

/// Selects a physical device object

/// Describes the requirements for to select a physical device
pub struct PhysicalDeviceSelectionRequirements {
	features_1_0: vk::PhysicalDeviceFeatures,
	features_1_1: vk::PhysicalDeviceVulkan11Features,
	features_1_2: vk::PhysicalDeviceVulkan12Features,
	features_1_3: vk::PhysicalDeviceVulkan13Features,
}

/// Struct to select a physical device
pub struct PhysicalDeviceSelector {
	instance: ash::Instance,
}

impl PhysicalDeviceSelector {

}