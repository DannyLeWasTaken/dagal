mod selector;

use ash::vk;
use crate::abstraction::prelude as abstraction;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct PhysicalDevice {
	/// Handle to [vk::PhysicalDevice]
	handle: vk::PhysicalDevice,
	/// Handle to [vk::PhysicalDeviceFeatures2]
	features: vk::PhysicalDeviceFeatures2,
	/// A vector containing all the [crate::abstraction::queue] of the physical device
	queues: Vec<abstraction::Queue>,
	/// Reference to [crate::abstraction::instance]
	instance: abstraction::Instance,
}

impl PhysicalDevice {
	pub fn new(instance: abstraction::Instance, physical_device: vk::PhysicalDevice) -> Self {

		// Get all features of the device
		let mut features_2 = vk::PhysicalDeviceFeatures2::default();
		features_2.features = vk::PhysicalDeviceFeatures::default();
		let mut feature_1_1 = vk::PhysicalDeviceVulkan11Features::default();
		features_2.p_next = abstraction::utility::p_next_mut(&mut feature_1_1);
		let mut feature_1_2 = vk::PhysicalDeviceVulkan12Features::default();
		feature_1_1.p_next = abstraction::utility::p_next_mut(&mut feature_1_2);
		let mut feature_1_3  = vk::PhysicalDeviceVulkan13Features::default();
		feature_1_2.p_next = abstraction::utility::p_next_mut(&mut feature_1_3);

		unsafe {
			instance.get_physical_device_features2(physical_device.clone(), &mut features_2);
		};

		Self {
			handle: physical_device,
			features: features_2,
			queues: Vec::new(),
			instance,
		}
	}

	pub fn get_queues(&self) {
		let mut queue_properties: Vec<vk::QueueFamilyProperties2> = Vec::new();
		unsafe {
			self.instance.handle
			    .get_physical_device_queue_family_properties2(self.handle, &mut queue_properties)
		};
	}
}