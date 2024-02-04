pub trait Resource {
    /// Get the descriptor of the struct of the resource
    fn get_descriptor(&self) -> crate::abstraction::descriptors::DescriptorInfo;
}
