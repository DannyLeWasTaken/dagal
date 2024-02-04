pub mod selector;
pub use selector::*;

use crate::abstraction::prelude as abstraction;
use ash::vk;
use ash::vk::Handle;
use std::collections::HashSet;
use std::ffi::CString;
use std::mem::swap;
use std::ptr;

#[derive(Clone)]
pub struct QueueFamilyInfo {
    /// Handle towards underlying data
    pub(crate) handle: vk::QueueFamilyProperties2,

    /// The family index of the queue family
    pub(crate) index: u32,

    /// Whether or not it is presentable in the queue
    pub(crate) presentable: bool,
}

#[derive(Clone)]
pub struct PhysicalDevice {
    /// Handle to [vk::PhysicalDevice]
    handle: vk::PhysicalDevice,
    /// Handle to [vk::PhysicalDeviceFeatures2]
    features: vk::PhysicalDeviceFeatures2,
    features_1_1: vk::PhysicalDeviceVulkan11Features,
    features_1_2: vk::PhysicalDeviceVulkan12Features,
    features_1_3: vk::PhysicalDeviceVulkan13Features,
    /// Extensions available on the [PhysicalDevice]
    extensions: Vec<vk::ExtensionProperties>,
    /// A vector containing all the [crate::abstraction::queue] of the physical device
    queues: Vec<QueueFamilyInfo>,
    /// Reference to [crate::abstraction::instance]
    instance: abstraction::Instance,
    /// Requirements listed out for the GPU
    gpu_requirements: PhysicalDeviceRequirements,
}
pub struct PhysicalDeviceFeatures {
    handle: vk::PhysicalDeviceFeatures2,
    features_1_1: vk::PhysicalDeviceVulkan12Features,
    features_1_2: vk::PhysicalDeviceVulkan12Features,
    features_1_3: vk::PhysicalDeviceVulkan13Features,
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

    pub fn new(
        instance: abstraction::Instance,
        physical_device: vk::PhysicalDevice,
        gpu_requirements: Option<selector::PhysicalDeviceRequirements>,
        surface: Option<abstraction::Surface>
    ) -> Option<Self> {
        // Get all features of the device
        let mut features_2 = vk::PhysicalDeviceFeatures2::default();
        features_2.features = vk::PhysicalDeviceFeatures::default();
        let mut features_1_1 = vk::PhysicalDeviceVulkan11Features::default();
        features_2.p_next = abstraction::utility::p_next_mut(&mut features_1_1);
        let mut features_1_2 = vk::PhysicalDeviceVulkan12Features::default();
        features_1_1.p_next = abstraction::utility::p_next_mut(&mut features_1_2);
        let mut features_1_3 = vk::PhysicalDeviceVulkan13Features::default();
        features_1_2.p_next = abstraction::utility::p_next_mut(&mut features_1_3);

        unsafe {
            instance
                .get_vk_instance()
                .get_physical_device_features2(physical_device.clone(), &mut features_2);
        };

        // Get all extensions of the device
        let extensions = unsafe {
            instance
                .get_vk_instance()
                .enumerate_device_extension_properties(physical_device.clone())
                .unwrap()
        };
        // Deal with dangling pointers
        features_2.p_next = ptr::null_mut();
        features_1_1.p_next = ptr::null_mut();
        features_1_2.p_next = ptr::null_mut();
        features_1_3.p_next = ptr::null_mut();

        // Get all queues of the physical device
        let queues = PhysicalDevice::retrieve_vk_queues(instance.get_vk_instance(), physical_device.clone());
        let queues: Vec<QueueFamilyInfo> = queues.into_iter().enumerate().map(|(index, queue)| {
            QueueFamilyInfo {
                handle: queue,
                index: index as u32,
                presentable: unsafe {
                    if let Some(surface) = surface {
                        if unsafe {
                            surface.get_loader().get_handle().get_handle().get_physical_device_surface_support(
                                physical_device,
                                index as u32,
                                surface.get_handle()
                            ).unwrap()
                        } {
                            true
                        }
                    }
                    false
                },
            }
        }).collect::<Vec<QueueFamilyInfo>>();
        let gpu_requirements_exist = gpu_requirements.is_some();
        let physical_device = Self {
            handle: physical_device,
            features: features_2,
            features_1_1,
            features_1_2,
            features_1_3,
            extensions,
            queues,
            instance: instance.clone(),
            gpu_requirements: gpu_requirements.clone().unwrap_or_default(),
        };
        // Ensure the most base requirements are met
        if !physical_device.meets_base_requirements() {
            return None;
        }
        if gpu_requirements_exist && !physical_device.meets_requirements(None) {
            return None;
        }
        Some(physical_device)
    }

    /// Checks if the base minimum requirements are even met in the first place
    pub(crate) fn meets_base_requirements(&self) -> bool {
        // We will be checking for the most basics of requirements
        if self.has_extensions(get_minimum_viable_requirements().extensions.as_slice()) {
            let features_1_1 = self.features_1_1;
            let features_1_2 = self.features_1_2;
            let features_1_3 = self.features_1_3;
            if features_1_2.buffer_device_address == vk::TRUE
                && features_1_2.descriptor_indexing == vk::TRUE
                && features_1_2.descriptor_binding_partially_bound == vk::TRUE
                && features_1_2.shader_sampled_image_array_non_uniform_indexing == vk::TRUE
                && features_1_2.shader_storage_image_array_non_uniform_indexing == vk::TRUE
                && features_1_2.shader_storage_buffer_array_non_uniform_indexing == vk::TRUE
                && features_1_2.descriptor_binding_sampled_image_update_after_bind == vk::TRUE
                && features_1_2.descriptor_binding_storage_image_update_after_bind == vk::TRUE
                && features_1_2.descriptor_binding_storage_buffer_update_after_bind == vk::TRUE
                && features_1_2.descriptor_binding_update_unused_while_pending == vk::TRUE
                && features_1_2.timeline_semaphore == vk::TRUE
            {
                return true;
            }
            return false;
        }
        false
    }

    /// Checks given a [selector::PhysicalDeviceRequirements] requirements, the current [PhysicalDevice] meets it
    pub fn meets_requirements(
        &self,
        requirements: Option<&selector::PhysicalDeviceRequirements>,
    ) -> bool {
        if let Some(requirements) = requirements {
            // check extensions
            self.has_extensions(requirements.extensions.as_slice());
        } else {
            // we're checking gpu_requirements that was initially passed in
            return self.meets_requirements(Some(&self.gpu_requirements));
        }
        false
    }

    pub unsafe fn get_handle(&self) -> &vk::PhysicalDevice {
        &self.handle
    }

    pub fn get_features(&self) -> (vk::PhysicalDeviceFeatures2, vk::PhysicalDeviceVulkan11Features, vk::PhysicalDeviceVulkan12Features, vk::PhysicalDeviceVulkan13Features) {
        (self.features, self.features_1_1, self.features_1_2, self.features_1_3)
    }

    pub fn get_extensions(&self) -> &[vk::ExtensionProperties] {
        self.extensions.as_slice()
    }

    pub fn get_gpu_requirements(&self) -> selector::PhysicalDeviceRequirements {
        self.gpu_requirements.clone()
    }

    pub fn queue_properties_exists(&self) {
        let mut queue_properties: Vec<vk::QueueFamilyProperties2> = Vec::new();
        unsafe {
            self.instance
                .get_vk_instance()
                .get_physical_device_queue_family_properties2(self.handle, &mut queue_properties)
        };
    }

    pub fn has_extensions<T: Into<String> + Clone>(&self, names: &[T]) -> bool {
        let mut required_extensions: HashSet<String> = HashSet::new();
        for name in names.iter() {
            required_extensions.insert(name.clone().into());
        }
        for extension_name in self.extensions.iter() {
            let name = abstraction::utility::vk_to_string(extension_name.extension_name.as_slice());
            if required_extensions.contains(&name) {
                required_extensions.remove(&name);
            }
        }
        required_extensions.is_empty()
    }

    pub fn get_queue_families(&self) -> &[vk::QueueFamilyProperties2] {
        self.queues.as_slice()
    }
}
