#[desc = "A rust package for libcurl."];
#[license = "MIT"];


use std::libc::c_char;
use std::c_str::CString;
use std::path::BytesContainer;
use std::str;


#[link(name = "curl")]
extern {
    fn curl_version() -> *c_char;
}


pub fn version() -> ~str {
    unsafe {
        // for curl version, we don't own it
        let cver = CString::new(curl_version(), false);
        str::from_utf8_owned(cver.container_into_owned_bytes())
    }
}

pub mod easy;
