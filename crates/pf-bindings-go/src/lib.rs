use pf_bindings_core::{register_gdp_package, verify_bet};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar};
use std::slice;

#[no_mangle]
pub extern "C" fn pf_verify_bet(
    receipt_json: *const c_char,
    transcript_json: *const c_char,
    error_out: *mut *mut c_char,
) -> c_int {
    if receipt_json.is_null() || transcript_json.is_null() {
        if !error_out.is_null() {
            let msg = CString::new("null pointer arguments").unwrap();
            unsafe {
                *error_out = msg.into_raw();
            }
        }
        return -1;
    }

    let receipt_str = unsafe { CStr::from_ptr(receipt_json).to_string_lossy() };
    let transcript_str = unsafe { CStr::from_ptr(transcript_json).to_string_lossy() };

    match verify_bet(&receipt_str, &transcript_str) {
        Ok(()) => 0,
        Err(err) => {
            if !error_out.is_null() {
                let msg = CString::new(err.to_string()).unwrap();
                unsafe {
                    *error_out = msg.into_raw();
                }
            }
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn pf_register_gdp_package(
    bytes: *const c_uchar,
    len: usize,
    error_out: *mut *mut c_char,
) -> c_int {
    if bytes.is_null() && len > 0 {
        if !error_out.is_null() {
            let msg = CString::new("null bytes pointer with non-zero length").unwrap();
            unsafe {
                *error_out = msg.into_raw();
            }
        }
        return -1;
    }

    let data = unsafe { slice::from_raw_parts(bytes, len) };

    match register_gdp_package(data) {
        Ok(()) => 0,
        Err(err) => {
            if !error_out.is_null() {
                let msg = CString::new(err.to_string()).unwrap();
                unsafe {
                    *error_out = msg.into_raw();
                }
            }
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn pf_free_error(error: *mut c_char) {
    if !error.is_null() {
        unsafe {
            let _ = CString::from_raw(error);
        }
    }
}
