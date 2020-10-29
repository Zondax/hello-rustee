use libc::c_char;
use std::ffi::{CStr, CString, NulError};

/// Wraps a C-string into a rust-friendly representation
/// no taking ownership of such C-string
pub fn string_from_ptr<'a>(ptr: *const c_char) -> Option<&'a CStr> {
    if !ptr.is_null() {
        unsafe { Some(CStr::from_ptr(ptr)) }
    } else {
        None
    }
}

/// Creates a C compatible string from a Rust String
pub fn str_to_c<T>(string: T) -> Result<CString, NulError>
where
    T: AsRef<str>,
{
    let string = string.as_ref();
    CString::new(string)
}
