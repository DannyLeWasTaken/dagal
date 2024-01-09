//! [self] can be thought as a most minimal abstraction layer for [ash]
//! # Lifetimes
//! Lifetimes are managed in either two ways:
//! - Reference counting using Arc
//! - [utility::deferred_deletion_queue]
//!
//! It is important to note that both CAN be used.

mod debugging;
pub mod descriptors;
pub mod device;
pub mod instance;
pub mod physical_device;
mod pipeline;
pub mod prelude;
pub mod queue;
pub mod resource;
pub mod utility;