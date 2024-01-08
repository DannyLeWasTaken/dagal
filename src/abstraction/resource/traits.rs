pub trait Resource {
    const MAX_CYCLES: u64 = 8;
    fn get_descriptor() -> crate::abstraction::descriptors::DescriptorInfo;
}
