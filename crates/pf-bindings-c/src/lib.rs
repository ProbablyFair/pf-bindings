use pf_bindings_core::{register_gdp_package, verify_bet};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar};
use std::ptr;
use std::slice;

/// Error codes for C API
pub const PF_SUCCESS: c_int = 0;
pub const PF_ERROR: c_int = -1;

/// Result codes for verification and registration
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PfResult {
    Success = 0,
    VerificationFailed = -1,
    RegistrationFailed = -2,
    InvalidInput = -3,
    InternalError = -4,
}

impl PartialEq<c_int> for PfResult {
    fn eq(&self, other: &c_int) -> bool {
        *self as c_int == *other
    }
}

/// Context for error messages
#[repr(C)]
pub struct PfError {
    pub message: *mut c_char,
}

impl PfError {
    pub fn from_string(error: String) -> Self {
        let msg = CString::new(error).unwrap();
        PfError {
            message: msg.into_raw() as *mut c_char,
        }
    }

    pub fn success() -> Self {
        PfError {
            message: ptr::null_mut(),
        }
    }
}

/// Verify a bet receipt against its transcript
///
/// # Parameters
/// - receipt_json: JSON string containing the bet receipt
/// - transcript_json: JSON string containing the transcript
/// - error_out: Output parameter for error message (can be null)
///
/// # Returns
/// PfResult::Success on success, error code otherwise
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

    let receipt_str = match unsafe { CStr::from_ptr(receipt_json).to_str() } {
        Ok(s) => s,
        Err(_) => {
            if !error_out.is_null() {
                unsafe {
                    (*error_out) =
                        PfError::from_string("invalid receipt JSON encoding".to_string());
                }
            }
            return PfResult::InvalidInput;
        }
    };

    let transcript_str = match unsafe { CStr::from_ptr(transcript_json).to_str() } {
        Ok(s) => s,
        Err(_) => {
            if !error_out.is_null() {
                unsafe {
                    (*error_out) =
                        PfError::from_string("invalid transcript JSON encoding".to_string());
                }
            }
            return PfResult::InvalidInput;
        }
    };

    match verify_bet(receipt_str, transcript_str) {
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
///
/// # Parameters
/// - bytes: Pointer to the GDP package binary data
/// - len: Length of the data in bytes
/// - error_out: Output parameter for error message (can be null)
///
/// # Returns
/// PfResult::Success on success, error code otherwise
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
///
/// # Parameters
/// - error: Error structure containing the message to free
#[no_mangle]
pub extern "C" fn pf_free_error(error: PfError) {
    if !error.message.is_null() {
        unsafe {
            let _ = CString::from_raw(error.message);
        }
    }
}

/// Get string representation of result code
///
/// # Parameters
/// - result: Result code
///
/// # Returns
/// Static string describing the result (do not free)
#[no_mangle]
pub extern "C" fn pf_result_string(result: PfResult) -> *const c_char {
    let string = match result {
        PfResult::Success => "Success",
        PfResult::VerificationFailed => "Verification failed",
        PfResult::RegistrationFailed => "Registration failed",
        PfResult::InvalidInput => "Invalid input",
        PfResult::InternalError => "Internal error",
    };

    string.as_ptr() as *const c_char
}

/// Get library version
///
/// # Returns
/// Static string containing version info (do not free)
#[no_mangle]
pub extern "C" fn pf_library_version() -> *const c_char {
    let version = "0.1.0";
    version.as_ptr() as *const c_char
}
