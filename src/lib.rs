
pub mod search_source;
pub mod get_torrent;

pub use search_source::search_source;
pub use get_torrent::get_torrent;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{c_char, CString, CStr};
    use serde_json::{to_string, from_str};

    // #[test]
    fn test_search_source_anime() {
        use recombox_plugin_provider::select_source::{InputPayload, OuputPayload};
        

        let new_input = InputPayload{
            id: String::from("1"),
            title: String::from("love is war"),
            source: String::from("anime"),
        };

        let input = CString::new(to_string(&new_input).unwrap()).unwrap();

        let result = search_source(input.as_ptr());

        let raw_output = unsafe { CString::from_raw(result) };
        let output: OuputPayload = from_str(raw_output.to_str().unwrap()).unwrap();

        println!("result: {:?}", output);

    }

    // #[test]
    fn test_search_source_movies() {
        use recombox_plugin_provider::select_source::{InputPayload, OuputPayload};
        

        let new_input = InputPayload{
            id: String::from("tt0145487"),
            title: String::from("Spider Man"),
            source: String::from("movies"),
        };

        let input = CString::new(to_string(&new_input).unwrap()).unwrap();

        let result = search_source(input.as_ptr());

        let raw_output = unsafe { CString::from_raw(result) };
        let output: OuputPayload = from_str(raw_output.to_str().unwrap()).unwrap();

        println!("result: {:?}", output);
    }

    // #[test]
    fn test_search_source_tv() {
        use recombox_plugin_provider::select_source::{InputPayload, OuputPayload};
        
        let new_input = InputPayload{
            id: String::from("tt9140554"),
            title: String::from("Loki"),
            source: String::from("tv"),
        };

        let input = CString::new(to_string(&new_input).unwrap()).unwrap();

        let result = search_source(input.as_ptr());

        let raw_output = unsafe { CString::from_raw(result) };
        let output: OuputPayload = from_str(raw_output.to_str().unwrap()).unwrap();

        println!("result: {:?}", output);
    }


    #[test]
    fn test_get_torrent_anime() {
        use recombox_plugin_provider::get_torrent::{InputPayload, OuputPayload};
        
        let new_input = InputPayload{
            id: String::from("65250016"),
            source: String::from("anime"),
        };

        let input = CString::new(to_string(&new_input).unwrap()).unwrap();

        let result = get_torrent(input.as_ptr());

        let raw_output = unsafe { CString::from_raw(result) };
        let output: OuputPayload = from_str(raw_output.to_str().unwrap()).unwrap();

        println!("result: {:?}", output);
    }
}
