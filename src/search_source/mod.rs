use std::ffi::CString;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn search_source() -> *mut c_char {
    // Run async reqwest inside a runtime
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all().build().unwrap();

    let body = rt.block_on(async {
        let resp = reqwest::get("https://httpbin.org/get")
            .await
            .expect("request failed");
        resp.text().await.expect("read body failed")
    });

    println!("body: {}", body);

    // Convert Rust String -> C string pointer
    let c_string = CString::new(body).unwrap();
    c_string.into_raw()
}
