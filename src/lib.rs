#![crate_name = "curl"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![desc = "A rust package for libcurl."]
#![license = "MIT"]
#![feature(phase)]

extern crate libc;
extern crate regex;

#[phase(plugin, link)] extern crate log;
#[phase(plugin)] extern crate regex_macros;

use libc::{c_char, c_long, c_int};
use std::c_str::CString;
use std::path::BytesContainer;

pub use self::easy::Curl as Curl;

#[link(name = "curl")]
extern {
    fn curl_version() -> *const c_char;
    fn curl_global_init(flags: c_long) -> c_int;
    fn curl_global_cleanup();
}

pub static GLOBAL_SSL : c_long = (1<<0);
pub static GLOBAL_WIN32 : c_long = (1<<1);
pub static GLOBAL_ALL : c_long = (GLOBAL_SSL|GLOBAL_WIN32);
pub static GLOBAL_NOTHING : c_long = 0;
pub static GLOBAL_DEFAULT : c_long = GLOBAL_ALL;
pub static GLOBAL_ACK_EINTR : c_long = (1<<2);

pub fn global_init(flags: c_long) -> int {
    unsafe { curl_global_init(flags) as int }
}

pub fn global_cleanup() {
    unsafe { curl_global_cleanup() }
}

pub fn version() -> String {
    unsafe {
        // for curl version, we don't own it
        let cver = CString::new(curl_version(), false);
        String::from_utf8(cver.container_into_owned_bytes()).unwrap()
    }
}

pub mod handlers;
pub mod http;
pub mod easy;
pub mod errors;
pub mod info;
pub mod opt;

#[cfg(test)]
mod test;
