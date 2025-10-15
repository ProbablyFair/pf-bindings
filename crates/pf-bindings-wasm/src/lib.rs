use wasm_bindgen::prelude::*;
use pf_bindings_core::{verify_bet as core_verify_bet, register_gdp_package as core_register_gdp_package, BindingError};

// Console logging for debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_js_value(s: &JsValue);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct PfError {
    message: String,
}

#[wasm_bindgen]
impl PfError {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl From<BindingError> for PfError {
    fn from(err: BindingError) -> Self {
        PfError {
            message: err.to_string(),
        }
    }
}

/// Verifies a bet receipt against its transcript
#[wasm_bindgen]
pub fn verify_bet(receipt_json: String, transcript_json: String) -> Result<(), JsValue> {
    console_log!("verify_bet called: receipt_len={}, transcript_len={}", 
                receipt_json.len(), transcript_json.len());
    
    match core_verify_bet(&receipt_json, &transcript_json) {
        Ok(()) => Ok(()),
        Err(err) => Err(JsValue::from(PfError::from(err))),
    }
}

/// Registers a GDP package for use in betting operations
#[wasm_bindgen]
pub fn register_gdp_package(bytes: &[u8]) -> Result<(), JsValue> {
    console_log!("register_gdp_package called: bytes_len={}", bytes.len());
    
    match core_register_gdp_package(bytes) {
        Ok(()) => Ok(()),
        Err(err) => Err(JsValue::from(PfError::from(err))),
    }
}

/// Convenience function for registering a GDP package from Uint8Array
#[wasm_bindgen]
pub fn register_gdp_package_uint8_array(bytes: &js_sys::Uint8Array) -> Result<(), JsValue> {
    console_log!("register_gdp_package_uint8_array called: bytes_len={}", bytes.length());
    
    let mut vec = vec![0u8; bytes.length() as usize];
    bytes.copy_to(&mut vec);
    
    match core_register_gdp_package(&vec) {
        Ok(()) => Ok(()),
        Err(err) => Err(JsValue::from(PfError::from(err))),
    }
}

// Note: Removed base64 function to avoid additional dependencies
// You can implement base64 decoding in JavaScript/WASM wrapper as needed

/// Get library version
#[wasm_bindgen]
pub fn library_version() -> String {
    "0.1.0".to_string()
}

/// Get supported features
#[wasm_bindgen]
pub fn supported_features() -> JsValue {
    let features = [
        "verify_bet",
        "register_gdp_package", 
        "register_gdp_package_uint8_array",
    ];
    
    // Manual serialization since we removed serde-wasm-bindgen
    let array = js_sys::Array::new();
    for feature in features.iter() {
        array.push(&JsValue::from_str(feature));
    }
    array.into()
}

// Export the bindings for use in Node.js or browser
#[wasm_bindgen(start)]
pub fn start() {
    console_log!("PF Bindings WebAssembly initialized");
}
