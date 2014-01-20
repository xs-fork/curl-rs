#[desc = "curl easy interface"];
#[license = "MIT"];

use std::libc::{c_void, uintptr_t};
use std::c_str::CString;
use std::path::BytesContainer;
use std::str;

#[link(name = "curl")]
extern {
    fn curl_easy_init() -> uintptr_t;
}

pub struct Curl {
    pub handle: uintptr_t,
}

impl Curl {
    pub fn is_null(&self) -> bool {
        self.handle == 0
    }
}


pub fn init() -> Curl {
    let hd = unsafe { curl_easy_init() };
    Curl { handle: hd }
}
