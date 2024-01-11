pub trait Resource {
    /// Get the descriptor of the struct of the resource
    fn get_descriptor() -> crate::abstraction::descriptors::DescriptorInfo;
}
