use std::libc::{uintptr_t, c_int, c_char, c_double, size_t, FILE};
use std::c_str::CString;
use std::path::BytesContainer;
use std::str;
use std::ptr;
use std::cast;

use opt;
// #[feature(link_args)]
// #[cfg(target_os = "macos")]
// #[link(args = "-o c-callback")]
// extern {
//     fn return_progress_function() -> uintptr_t;
// }


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
    fn curl_slist_append(list: uintptr_t, string: *c_char) -> uintptr_t;
}

pub trait ToCurlOptParam {
    fn to_curl_opt_param(&self) -> uintptr_t;
}

impl ToCurlOptParam for uintptr_t {
    fn to_curl_opt_param(&self) -> uintptr_t {
        *self
    }
}


impl ToCurlOptParam for int {
    fn to_curl_opt_param(&self) -> uintptr_t {
        *self as uintptr_t
    }
}

impl ToCurlOptParam for bool {
    fn to_curl_opt_param(&self) -> uintptr_t {
        match *self {
            true  => 1,
            false => 0
        }
    }
}

impl<'a> ToCurlOptParam for &'a str {
    fn to_curl_opt_param(&self) -> uintptr_t {
        let c_string = self.to_c_str();
        ptr::to_unsafe_ptr(&c_string.container_into_owned_bytes()[0]) as uintptr_t
    }
}

// NOTE: return [u8] as a *c_char will not guarantee a \0 byte at end.
//       So here I convert it to a CString.
impl<'a> ToCurlOptParam for &'a [u8] {
    fn to_curl_opt_param(&self) -> uintptr_t {
        let c_string = self.to_c_str();
        ptr::to_unsafe_ptr(&c_string.container_into_owned_bytes()[0]) as uintptr_t
    }
}

impl ToCurlOptParam for ~[~str] {
    fn to_curl_opt_param(&self) -> uintptr_t {
        self.iter().fold(0, |acc, item| {
                item.to_c_str().with_ref(|s| {
                        unsafe { curl_slist_append(acc, s) }
                    })
            })
    }
}

impl<'r> ToCurlOptParam for 'r |f64,f64,f64,f64| -> int {
    fn to_curl_opt_param(&self) -> uintptr_t {
        ptr::to_unsafe_ptr(self) as uintptr_t
    }
}

impl ToCurlOptParam for *FILE {
    fn to_curl_opt_param(&self) -> uintptr_t {
        unsafe { cast::transmute(*self) }
    }
}

// Curl

pub struct Curl {
    handle: uintptr_t,
}

impl Drop for Curl {
    fn drop(&mut self) {
        unsafe { curl_easy_cleanup(self.handle) }
    }
}


// TODO: add deriving
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
                    str::from_utf8_owned(escaped_bytes).unwrap()
                }
            })
    }

    pub fn init() -> Curl {
        let hd = unsafe { curl_easy_init() };
        Curl { handle: hd }
    }

    /// empty func; use Drop trait instead
    pub fn cleanup(&self) {
        // unsafe { curl_easy_cleanup(self.handle) }
    }

    pub fn duphandle(&self) -> Curl {
        let ret = unsafe { curl_easy_duphandle(self.handle) };
        Curl { handle: ret }
    }

    pub fn perform(&self) -> int {
        let ret = unsafe { curl_easy_perform(self.handle) };
        ret as int
    }

    pub fn setopt<T: ToCurlOptParam>(&self, option: c_int, param: T) -> int {
        match option {
            opt::PROGRESSFUNCTION =>
                unsafe { curl_easy_setopt(self.handle, option, cast::transmute(curl_cb_progress_fn)) as int },
            opt::WRITEFUNCTION =>
                unsafe { curl_easy_setopt(self.handle, option, cast::transmute(curl_cb_write_fn)) as int },
            _ =>
                unsafe { curl_easy_setopt(self.handle, option, param.to_curl_opt_param()) as int }
        }
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

pub fn cleanup(_handle: Curl) {
    // let hd = handle.handle;
    // unsafe { curl_easy_cleanup(hd) }
}


pub fn perform(handle: Curl) {
    let hd = handle.handle;
    unsafe { curl_easy_perform(hd) };
}

pub fn strerror(code: int) -> ~str {
    unsafe {
        let cver = CString::new(curl_easy_strerror(code as c_int), false);
        str::from_utf8_owned(cver.container_into_owned_bytes()).unwrap()
    }
}


// Callback

pub extern "C" fn curl_cb_progress_fn(clientp: uintptr_t, dltotal: c_double,  dlnow: c_double,
                                      ultotal: c_double, ulnow: c_double) -> c_int {
    print!("\x08\x08\x08\x08\x08\x08\x08bnow: = {}%\r", dlnow/dltotal*100f64);
    0 as c_int
}

// size_t function( char *ptr, size_t size, size_t nmemb, void *userdata);
pub extern "C" fn curl_cb_write_fn(p: *c_char, size: size_t, nmemb: size_t, userdata: uintptr_t) -> size_t {
    size * nmemb
}

// size_t function( void *ptr, size_t size, size_t nmemb, void *userdata);
pub extern "C" fn curl_cb_write_fn(p: *c_char, size: size_t, nmemb: size_t, userdata: uintptr_t) -> size_t {
    size * nmemb
}
