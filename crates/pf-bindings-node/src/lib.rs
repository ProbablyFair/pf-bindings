use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn verify_bet(receipt_json: String, transcript_json: String) -> Result<()> {
    pf_bindings_core::verify_bet(&receipt_json, &transcript_json)
        .map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
pub fn register_gdp_package(bytes: Buffer) -> Result<()> {
    pf_bindings_core::register_gdp_package(&bytes)
        .map_err(|err| Error::from_reason(err.to_string()))
}
