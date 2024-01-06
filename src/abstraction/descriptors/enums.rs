use ash::vk;

pub enum DescriptorInfo {
	Image(vk::DescriptorImageInfo),
	Buffer(vk::DescriptorBufferInfo),
}