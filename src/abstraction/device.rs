use std::sync::Arc;
use ash::vk;
use ash::vk::TaggedStructure;

/// Abstraction for the Vulkan device of Vulkan
#[derive(Clone)]
pub struct Device {
	handle: Arc<ash::Device>,
}

impl Device {
	pub fn new(instance: ash::Instance, physical_device: vk::PhysicalDevice) -> Result<Arc<Device>, vk::Result> {
		let device_ci = vk::DeviceCreateInfo {
			s_type: vk::DeviceCreateInfo::STRUCTURE_TYPE,

			flags: Default::default(),
			queue_create_info_count: 0,
			p_queue_create_infos: (),
			enabled_extension_count: 0,
			pp_enabled_extension_names: (),
			p_enabled_features: (),

			..Default::default()
		};
		let handle = unsafe {
			instance.create_device(physical_device.clone(), &device_ci, None)
		};
		if handle.is_err() {
			return Err(handle.err().unwrap());
		}
		let handle = Arc::new(handle.unwrap());
		Ok(Arc::new(Self {
			handle,
		}))
	}

	pub fn handle_as_ref(&self) -> &ash::Device {
		&self.handle
	}
}

impl Drop for Device {
	fn drop(&mut self) {
		unsafe {
			self.handle.destroy_device(None);
		}
	}
}