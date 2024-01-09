//! This module allows for quicker initialization of devices.

use crate::abstraction::prelude as abstraction;

/// [DeviceBuilder] makes various assumptions about the device building process.
///
/// Firstly, we assume that all [abstraction::PhysicalDeviceRequirements] will also be used in
/// [abstraction::Device].
///
/// This means all [abstraction::QueueRequirements] passed, must also be used in [abstraction::Device].
pub struct DeviceBuilder {}
