use std::sync::Arc;
use ash::vk;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle, WindowHandle};
use crate::abstraction::prelude as abstraction;


#[derive(Clone)]
struct SurfaceLoaderInner {
	handle: ash::extensions::khr::Surface,
}

impl SurfaceLoaderInner {
	pub fn get_handle(&self) -> ash::extensions::khr::Surface {
		self.handle.clone()
	}
}

#[derive(Clone)]
pub struct SurfaceLoader {
	handle: Arc<SurfaceLoaderInner>,
}

impl SurfaceLoader {
	pub fn get_handle(&self) -> Arc<SurfaceLoaderInner> {
		self.handle.clone()
	}
}

pub struct Surface {
	handle: vk::SurfaceKHR,
	loader: SurfaceLoader,
}

impl Surface {
	pub fn new(instance: &abstraction::Instance, loader: SurfaceLoader, raw_display_handle: RawDisplayHandle, raw_window_handle: RawWindowHandle) -> Self {
		let handle = unsafe {
			ash_window::create_surface(instance.get_vk_entry(), instance.get_vk_instance(), raw_display_handle, raw_window_handle, None).unwrap()
		};
		Self {
			handle,
			loader,
		}
	}

	pub fn get_loader(&self) -> SurfaceLoader {
		self.loader.clone()
	}

	pub fn get_handle(&self) -> vk::SurfaceKHR {
		self.handle
	}
}

impl Drop for Surface {
	fn drop(&mut self) {
		unsafe {
			self.loader.handle.get_handle().destroy_surface(self.handle, None);
		}
	}
}