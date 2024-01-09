mod selector;

use std::collections::HashSet;
use crate::abstraction::prelude as abstraction;
use ash::vk;
use ash::vk::{Handle};

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct PhysicalDevice {
    /// Handle to [vk::PhysicalDevice]
    handle: vk::PhysicalDevice,
    /// Handle to [vk::PhysicalDeviceFeatures2]
    features: vk::PhysicalDeviceFeatures2,
    /// Extensions available on the [PhysicalDevice]
    extensions: Vec<vk::ExtensionProperties>,
    /// A vector containing all the [crate::abstraction::queue] of the physical device
    queues: Vec<vk::QueueFamilyProperties2>,
    /// Reference to [crate::abstraction::instance]
    instance: abstraction::Instance,
}

impl PhysicalDevice {
    /// Retrieve all queues
    fn retrieve_vk_queues(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties2> {
        let mut queues = Vec::new();
        unsafe {
            instance.get_physical_device_queue_family_properties2(
                physical_device,
                queues.as_mut_slice(),
            )
        }
        queues
    }

    pub fn new(instance: abstraction::Instance, physical_device: vk::PhysicalDevice) -> Self {
        // Get all features of the device
        let mut features_2 = vk::PhysicalDeviceFeatures2::default();
        features_2.features = vk::PhysicalDeviceFeatures::default();
        let mut feature_1_1 = vk::PhysicalDeviceVulkan11Features::default();
        features_2.p_next = abstraction::utility::p_next_mut(&mut feature_1_1);
        let mut feature_1_2 = vk::PhysicalDeviceVulkan12Features::default();
        feature_1_1.p_next = abstraction::utility::p_next_mut(&mut feature_1_2);
        let mut feature_1_3 = vk::PhysicalDeviceVulkan13Features::default();
        feature_1_2.p_next = abstraction::utility::p_next_mut(&mut feature_1_3);

        unsafe {
            instance.get_physical_device_features2(physical_device.clone(), &mut features_2);
        };

        // Get all extensions of the device
        let extensions = unsafe {
            instance
                .handle
                .enumerate_device_extension_properties(physical_device.clone())
                .unwrap()
        };

        // Get all queues of the physical device
        let queues = PhysicalDevice::retrieve_vk_queues(&instance.handle, physical_device.clone());

        Self {
            handle: physical_device,
            features: features_2,
            extensions,
            queues,
            instance,
        }
    }

    pub unsafe fn get_handle(&self) -> &vk::PhysicalDevice {
        &self.handle
    }

    pub fn get_features(&self) -> vk::PhysicalDeviceFeatures2 {
        self.features
    }

    pub fn get_extensions(&self) -> &[vk::ExtensionProperties] {
        self.extensions.as_slice()
    }

    /// Requests a queue with the given flags
    /// `unique_queue` indicates if the queue should be entirely unique or not.
    /// `quantity` indicates the # of said queue in question
    pub fn request_queue(&mut self, flags: vk::QueueFlags, unique_queue: bool, quantity: u32) {

    }

    pub fn queue_properties_exists(&self) {
        let mut queue_properties: Vec<vk::QueueFamilyProperties2> = Vec::new();
        unsafe {
            self.instance
                .handle
                .get_physical_device_queue_family_properties2(self.handle, &mut queue_properties)
        };
    }

    pub fn has_extensions<T: Into<String>>(&self, names: &[T]) -> bool {
        let mut required_extensions: HashSet<String> = HashSet::new();
        for name in names.iter() {
            required_extensions.insert(name.into());
        }
        for extension_name in self.extensions.iter() {
            if required_extensions.contains(extension_name.extension_name.into()) {
                required_extensions.remove(extension_name.extension_name.into())
            }
        }
        required_extensions.is_empty()
    }

    pub fn get_queues(&self) -> &[vk::QueueFamilyProperties2] {
        self.queues.as_slice()
    }
}
