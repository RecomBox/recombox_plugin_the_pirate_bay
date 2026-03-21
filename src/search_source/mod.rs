use std::{ffi::{CStr, CString, c_char}, ptr::null_mut};
use serde_json::{to_string};
use serde::{Deserialize, Serialize};
use recombox_plugin_provider::select_source::{InputPayload, OuputPayloadInfo, OuputPayload};


mod anime;
mod movies;
mod tv;



#[unsafe(no_mangle)]
pub extern "C" fn search_source(input: *const c_char) -> *mut c_char {
    // Convert C char into -> Rust Struct
    let rust_str = unsafe { CStr::from_ptr(input).to_str().unwrap() };
    let input_payload: InputPayload = serde_json::from_str(rust_str).unwrap();

    // Run async inside a runtime
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all().build().unwrap();

    let result = rt.block_on(async {
        match input_payload.source.to_lowercase().as_str() {
            "anime" => {
                return anime::new(input_payload).await;
            }
            "movies" => {
                return movies::new(input_payload).await;
            }
            "tv" => {
                return tv::new(input_payload).await;
            }
            _ => {
                return Err(anyhow::Error::msg("Invalid 'source'"));
            }
        };
        
    });
    match result {
        Ok(output) => {
            let output_str = to_string(&output).unwrap();
            let c_string = CString::new(output_str).unwrap();
            return c_string.into_raw();
        },
        Err(e) => {
            eprintln!("{:?}", e);
            return null_mut();
        }
    };
}
