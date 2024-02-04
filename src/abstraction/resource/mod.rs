/// represents all possible resources in vulkan.
/// any struct that implements [resource] trait must always contain an inner
/// struct which represent that actual underlying representation of the struct.
/// this is due to the
pub mod traits;
pub mod buffer;
pub mod image;

pub use traits::*;
pub use buffer::*;