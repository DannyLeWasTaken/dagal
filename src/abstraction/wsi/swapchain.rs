/*
* Abstracts the [vk::SwapchainKHR]
*/

use std::sync::Arc;
use ash::vk;
use crate::abstraction::prelude as abstraction;

struct SwapchainLoaderInner {
	handle: ash::extensions::khr::Swapchain,
	device: abstraction::Device,
}

impl SwapchainLoaderInner {
	pub fn get_handle(&self) -> ash::extensions::khr::Swapchain {
		self.handle.clone()
	}
}

#[derive(Copy, Clone)]
pub struct SwapchainLoader {
	handle: Arc<SwapchainLoaderInner>
}

impl SwapchainLoader {
	pub fn new(instance: &abstraction::Instance, device: abstraction::Device) -> Self {
		let handle = unsafe {
			ash::extensions::khr::Swapchain::new(instance.get_vk_instance(), device.handle_as_ref())
		};
		Self {
			handle: Arc::new(SwapchainLoaderInner {
				handle,
				device,
			})
		}
	}

	pub fn get_handle(&self) -> SwapchainLoaderInner {
		self.get_handle()
	}
}

pub struct Swapchain {
	handle: vk::SwapchainKHR,
	device: abstraction::Device,
}

impl Swapchain {

	pub fn get_handle(&self) -> vk::SwapchainKHR {
		self.handle
	}

	pub fn get_device(&self) -> abstraction::Device {
		self.device.clone()
	}
}

impl Drop for Swapchain {
	fn drop(&mut self) {

	}
}
