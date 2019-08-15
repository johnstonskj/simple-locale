use std::ffi::CStr;
use std::os::raw::c_char;

use super::langinfo::{nl_item, nl_langinfo};

/// Convert a raw C string, `*mut c_char` into a Rust String
/// type. This function should only be called from unsafe code.
pub unsafe fn cstr_to_string(c_str: *mut c_char) -> String {
    CStr::from_ptr(c_str).to_string_lossy().into_owned()
}

/// Constructs a grouping vector from the `lconv` POSIX API,
/// it treats each character as a numeric value, terminated
/// by either zero or MAX_CHAR. This function should only be
/// called from unsafe code.
pub unsafe fn grouping_vector(grouping: *mut c_char) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut c_ptr = grouping;
    loop {
        let a_char = *c_ptr;
        result.push(a_char as usize);
        if a_char == 0 || a_char == c_char::max_value() {
            break;
        }
        c_ptr = c_ptr.add(1);
    }
    result
}

/// A complete wrapper around the POSIX `nl_langinfo` call,
/// it encapsulates all unsafe code and returns a Rust
/// String, or None if the call returned an empty string.
pub fn get_nl_string(item: u32) -> Option<String> {
    let r_str = unsafe {
        let c_str: *mut c_char = nl_langinfo(item as nl_item);
        cstr_to_string(c_str)
    };
    if r_str.is_empty() {
        None
    } else {
        Some(r_str)
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::ffi::utils::get_nl_string;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_all_nl_settings() {
        for i in 0..58 {
            let value = get_nl_string(i);
            println!("{:#?}", value);
        }
    }
}
