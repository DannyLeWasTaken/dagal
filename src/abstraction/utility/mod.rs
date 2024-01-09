pub mod deferred_deletion_queue;
pub mod shader;
pub mod storage;
pub mod traits;

use std::ffi::{c_char, c_void, CStr};

/// Allows us to quickly get the respective p_next pointers
pub fn p_next<T>(data: &T) -> *const c_void {
    data as *const _ as *const c_void
}

/// Allows us to quickly get the respective p_next pointers
pub fn p_next_mut<T>(data: &mut T) -> *mut c_void {
    data as *mut _ as *mut c_void
}

/// Convert any raw string array to [String]
pub fn vk_to_string(raw_string_array: &[c_char]) -> String {
    let raw_string = unsafe { CStr::from_ptr(raw_string_array.as_ptr()) };
    raw_string.to_str().unwrap().to_owned()
}
