use ash::vk;
use crate::abstraction::descriptors::DescriptorInfo;
use crate::abstraction::prelude as abstraction;
use crate::abstraction::resource::Resource;


pub struct Image {
	handle: vk::Image,
	device: abstraction::Device,
	sampler: vk::Sampler,
	image_view: vk::ImageView,
	image_layout: vk::ImageLayout,
}

impl Image {
	pub fn from_vk(image: vk::Image, device: abstraction::Device, sampler: vk::Sampler, image_view: vk::ImageView, image_layout: vk::ImageLayout) -> Self {
		Self {
			handle: image,
			device,
			sampler,
			image_view,
			image_layout
		}
	}
}

impl Resource for Image {
	fn get_descriptor(&self) -> DescriptorInfo {
		DescriptorInfo::Image(vk::DescriptorImageInfo {
			sampler: self.sampler,
			image_view: self.image_view,
			image_layout: self.image_layout,
		})
	}
}

impl Drop for Image {
	fn drop(&mut self) {
		unsafe {
			self.device.handle_as_ref().destroy_image(self.handle, None);
		};
	}
}