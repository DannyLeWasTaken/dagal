/// Represents all possible resources in Vulkan.
/// Any struct that implements [Resource] trait must always contain an INNER
/// struct which represent that actual underlying representation of the struct.
/// This is due to the
pub mod traits;
pub use traits::*;
