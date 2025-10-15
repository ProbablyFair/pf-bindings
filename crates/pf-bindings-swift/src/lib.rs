use pf_bindings_core::{register_gdp_package, verify_bet};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar};
use std::ptr;
use std::slice;

/// Result codes C-compatible enum for Swift compatibility
#[repr(C)]
#[derive(Debug)]
pub enum PfResult {
    Success = 0,
    VerificationFailed = -1,
    RegistrationFailed = -2,
    InvalidInput = -3,
    InternalError = -4,
}

/// Error context structure for Swift compatibility
#[repr(C)]
pub struct PfError {
    pub message: *mut c_char,
}

impl PfError {
    pub fn from_string(error: String) -> Self {
        let msg = CString::new(error).unwrap();
        PfError {
            message: msg.into_raw(),
        }
    }

    pub fn success() -> Self {
        PfError {
            message: ptr::null_mut(),
        }
    }
}

/// Verify a bet receipt against its transcript
#[no_mangle]
pub extern "C" fn pf_verify_bet(
    receipt_json: *const c_char,
    transcript_json: *const c_char,
    error_out: *mut PfError,
) -> PfResult {
    if receipt_json.is_null() || transcript_json.is_null() {
        if !error_out.is_null() {
            unsafe {
                (*error_out) = PfError::from_string("null pointer arguments".to_string());
            }
        }
        return PfResult::InvalidInput;
    }

    let receipt_str = unsafe { CStr::from_ptr(receipt_json).to_string_lossy() };
    let transcript_str = unsafe { CStr::from_ptr(transcript_json).to_string_lossy() };

    match verify_bet(&receipt_str, &transcript_str) {
        Ok(()) => {
            if !error_out.is_null() {
                unsafe {
                    (*error_out) = PfError::success();
                }
            }
            PfResult::Success
        }
        Err(err) => {
            if !error_out.is_null() {
                unsafe {
                    (*error_out) = PfError::from_string(err.to_string());
                }
            }
            PfResult::VerificationFailed
        }
    }
}

/// Register a GDP package for use in betting operations
#[no_mangle]
pub extern "C" fn pf_register_gdp_package(
    bytes: *const c_uchar,
    len: usize,
    error_out: *mut PfError,
) -> PfResult {
    if bytes.is_null() && len > 0 {
        if !error_out.is_null() {
            unsafe {
                (*error_out) =
                    PfError::from_string("null bytes pointer with non-zero length".to_string());
            }
        }
        return PfResult::InvalidInput;
    }

    let data = unsafe { slice::from_raw_parts(bytes, len) };

    match register_gdp_package(data) {
        Ok(()) => {
            if !error_out.is_null() {
                unsafe {
                    (*error_out) = PfError::success();
                }
            }
            PfResult::Success
        }
        Err(err) => {
            if !error_out.is_null() {
                unsafe {
                    (*error_out) = PfError::from_string(err.to_string());
                }
            }
            PfResult::RegistrationFailed
        }
    }
}

/// Free error message memory
#[no_mangle]
pub extern "C" fn pf_free_error(error: PfError) {
    if !error.message.is_null() {
        unsafe {
            let _ = CString::from_raw(error.message);
        }
    }
}

/// Get library version
#[no_mangle]
pub extern "C" fn pf_library_version() -> *const c_char {
    let version = "0.1.0";
    version.as_ptr() as *const c_char
}
