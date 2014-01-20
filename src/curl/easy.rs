use std::libc::{uintptr_t, c_int, c_char};
use std::c_str::CString;
use std::path::BytesContainer;
use std::str;


#[link(name = "curl")]
extern {
    fn curl_easy_escape(h: uintptr_t, url: *c_char, length: c_int) -> *c_char;
    fn curl_easy_init() -> uintptr_t;
    fn curl_easy_cleanup(h: uintptr_t);
    fn curl_easy_duphandle(h: uintptr_t) -> uintptr_t;
    fn curl_easy_perform(h: uintptr_t) -> c_int;
    fn curl_easy_reset(h: uintptr_t);
    fn curl_easy_strerror(code: c_int) -> *c_char;
    fn curl_easy_setopt(h: uintptr_t, option: c_int, parameter: uintptr_t) -> c_int;
    fn curl_easy_unescape(h: uintptr_t, url: *c_char, inlength: c_int, outlength: *c_int) -> *c_char;
    fn curl_free(ptr: *c_char);
}

pub struct Curl {
    handle: uintptr_t,
}

impl Curl {
    pub fn is_null(&self) -> bool {
        self.handle == 0
    }

    // FIXME: handle \x00 byte in string
    pub fn escape(&self, url: &str) -> ~str {
        let c_url = url.to_c_str();
        c_url.with_ref(|c_buf| {
                unsafe {
                    let ret = curl_easy_escape(self.handle, c_buf, url.len() as c_int);
                    let escaped_bytes = CString::new(ret, false).container_into_owned_bytes();
                    curl_free(ret);
                    str::from_utf8_owned(escaped_bytes)
                }
            })
    }

    pub fn init() -> Curl {
        let hd = unsafe { curl_easy_init() };
        Curl { handle: hd }
    }

    pub fn cleanup(&self) {
        unsafe { curl_easy_cleanup(self.handle) }
    }

    pub fn duphandle(&self) -> Curl {
        let ret = unsafe { curl_easy_duphandle(self.handle) };
        Curl { handle: ret }
    }

    pub fn perform(&self) -> int {
        let ret = unsafe { curl_easy_perform(self.handle) };
        ret as int
    }

    pub fn reset(&self) {
        unsafe { curl_easy_reset(self.handle) }
    }

    pub fn unescape(&self, url: &str) -> ~str {
        let c_url = url.to_c_str();
        let outlen: c_int = 0;  // does not need to be mut
        c_url.with_ref(|c_buf| {
                unsafe {
                    let ret = curl_easy_unescape(self.handle, c_buf, url.len() as c_int, &outlen);
                    let unescaped_url = str::raw::from_buf_len(ret as *u8, outlen as uint);
                    curl_free(ret);
                    unescaped_url
                }
            })
    }

}

pub fn init() -> Curl {
    let hd = unsafe { curl_easy_init() };
    Curl { handle: hd }
}

pub fn cleanup(handle: Curl) {
    let hd = handle.handle;
    unsafe { curl_easy_cleanup(hd) }
}


pub fn perform(handle: Curl) {
    let hd = handle.handle;
    unsafe { curl_easy_perform(hd) };
}

pub fn strerror(code: int) -> ~str {
    unsafe {
        let cver = CString::new(curl_easy_strerror(code as c_int), false);
        str::from_utf8_owned(cver.container_into_owned_bytes())
    }
}
