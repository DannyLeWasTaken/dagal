pub mod storage;
pub mod shader;
pub mod deferred_deletion_queue;

use std::ffi::c_void;

/// Allows us to quickly get the respective p_next pointers
pub fn p_next<T>(data: &T) -> *const c_void {
	data as *const _ as *const c_void
}

/// Allows us to quickly get the respective p_next pointers
pub fn p_next_mut<T>(data: &mut T) -> *mut c_void {
	data as *mut _ as *mut c_void
}