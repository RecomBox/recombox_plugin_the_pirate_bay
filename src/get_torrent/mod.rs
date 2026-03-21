use std::{ffi::{CStr, CString, c_char}, ptr::null_mut};
use serde_json::{to_string, Value};
use reqwest::Client;
use urlencoding::encode;

use recombox_plugin_provider::get_torrent::{InputPayload, OuputPayloadInfo, OuputPayload};


pub const TRACKERS: [&str; 13] = [
    "udp://tracker.opentrackr.org:1337",
    "udp://open.stealth.si:80/announce",
    "udp://tracker.torrent.eu.org:451/announce",
    "udp://tracker.bittor.pw:1337/announce",
    "udp://public.popcorn-tracker.org:6969/announce",
    "udp://tracker.dler.org:6969/announce",
    "udp://exodus.desync.com:6969",
    "udp://open.demonii.com:1337/announce",
    "udp://glotorrents.pw:6969/announce",
    "udp://tracker.coppersurfer.tk:6969",
    "udp://torrent.gresille.org:80/announce",
    "udp://p4p.arenabg.com:1337",
    "udp://tracker.internetwarriors.net:1337",
];


#[unsafe(no_mangle)]
pub extern "C" fn get_torrent(input: *const c_char) -> *mut c_char {
    // Convert C char into -> Rust Struct
    let rust_str = unsafe { CStr::from_ptr(input).to_str().unwrap() };
    let input_payload: InputPayload = serde_json::from_str(rust_str).unwrap();

    // Run async inside a runtime
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all().build().unwrap();

    let result = rt.block_on(async {
        return new(input_payload).await;

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


pub async fn new(input: InputPayload) -> anyhow::Result<OuputPayload>{
    
    let client = Client::new();
    let res = client.get(format!("https://apibay.org/t.php?id={}", input.id))
        .send()
        .await?;

    let data: Value = res.json().await?;


    let mut new_output_payload = OuputPayload(vec![]);


    let title = data.get("name")
        .ok_or("Unable to get title")
        .map_err(|e| anyhow::Error::msg(e))?
        .as_str()
        .ok_or("Unable to convert to str")
        .map_err(|e| anyhow::Error::msg(e))?
        .to_string();

    let info_hash = data.get("info_hash")
        .ok_or("Unable to get info_hash")
        .map_err(|e| anyhow::Error::msg(e))?
        .as_str()
        .ok_or("Unable to convert to str")
        .map_err(|e| anyhow::Error::msg(e))?
        .to_string();

    let mut torrent_url = format!(
        "magnet:?xt=urn:btih:{}&dn={}",
        info_hash,
        encode(&title)
    );

    for tr in TRACKERS {
        torrent_url.push_str("&tr=");
        torrent_url.push_str(&urlencoding::encode(tr));
    }

    new_output_payload.0.push(OuputPayloadInfo{
        title,
        torrent_url,
    });


    return Ok(new_output_payload);
    

    // return Err(anyhow::Error::msg("Not implemented"));

}