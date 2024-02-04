use crate::abstraction::allocators::Allocator;
use crate::ash::vk;
use crate::abstraction::prelude as abstraction;

/*
* An abstraction for [vk::Buffer]
*/

#[derive(Clone)]
pub struct Buffer {
	handle: vk::Buffer,
}

impl Buffer {
	pub fn new(instance: &abstraction::Instance, allocator: &dyn Allocator) -> Self {
		Self {

		}
	}
}