use jni::JNIEnv;
use jni::objects::{JClass, JString, JByteArray};
use jni::sys::jint;
use pf_bindings_core::{verify_bet, register_gdp_package};
use std::panic::{catch_unwind, AssertUnwindSafe};

#[no_mangle]
pub extern "system" fn Java_com_probablyfair_PfBindings_verifyBet(
    mut env: JNIEnv,
    _class: JClass,
    receipt_json: JString,
    transcript_json: JString,
) -> jint {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let receipt_str: String = env.get_string(&receipt_json)?.into();
        let transcript_str: String = env.get_string(&transcript_json)?.into();
        
        match verify_bet(&receipt_str, &transcript_str) {
            Ok(()) => Ok(()),
            Err(e) => {
                let msg = e.to_string();
                env.throw_new("java/lang/Exception", &msg).unwrap();
                Err(jni::errors::Error::JavaException)
            }
        }
    }));
    
    match result {
        Ok(Ok(())) => 0,
        Ok(Err(_)) | Err(_) => -1,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_probablyfair_PfBindings_registerGdpPackage(
    mut env: JNIEnv,
    _class: JClass,
    bytes: JByteArray,
) -> jint {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let byte_array = env.convert_byte_array(&bytes)?;
        
        match register_gdp_package(&byte_array) {
            Ok(()) => Ok(()),
            Err(e) => {
                let msg = e.to_string();
                env.throw_new("java/lang/Exception", &msg).unwrap();
                Err(jni::errors::Error::JavaException)
            }
        }
    }));
    
    match result {
        Ok(Ok(())) => 0,
        Ok(Err(_)) | Err(_) => -1,
    }
}
