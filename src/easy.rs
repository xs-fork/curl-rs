use libc::{uintptr_t, c_int, c_char, c_double, size_t, c_long, FILE, c_void};
use std::c_str::CString;
use std::str;
use std::mem;
use std::to_str::ToStr;

use opt;

#[allow(dead_code)]
#[allow(unused_variable)]
#[link(name = "curl")]
extern {
    fn curl_easy_escape(h: uintptr_t, url: *c_char, length: c_int) -> *c_char;
    fn curl_easy_init() -> uintptr_t;
    fn curl_easy_cleanup(h: uintptr_t);
    fn curl_easy_duphandle(h: uintptr_t) -> uintptr_t;
    fn curl_easy_getinfo(h: uintptr_t, inf: c_int, ptr: *mut c_void) -> c_int;
    fn curl_easy_perform(h: uintptr_t) -> c_int;
    fn curl_easy_reset(h: uintptr_t);
    fn curl_easy_strerror(code: c_int) -> *c_char;
    fn curl_easy_setopt(h: uintptr_t, option: c_int, parameter: uintptr_t) -> c_int;
    fn curl_easy_unescape(h: uintptr_t, url: *c_char, inlength: c_int, outlength: *c_int) -> *c_char;
    fn curl_free(ptr: *c_char);
    fn curl_slist_append(list: uintptr_t, string: *c_char) -> uintptr_t;
    fn curl_slist_free_all(list: uintptr_t);
}

pub trait ToCurlOptParam {
    fn with_curl_opt_param(&self, f:|x: uintptr_t|);
}

impl ToCurlOptParam for uintptr_t {
    fn with_curl_opt_param(&self, f:|x: uintptr_t|) {
        f(*self)
    }
}

impl ToCurlOptParam for int {
    fn with_curl_opt_param(&self, f:|x: uintptr_t|) {
        f(*self as uintptr_t)
    }
}

impl ToCurlOptParam for bool {
    fn with_curl_opt_param(&self, f:|x: uintptr_t|) {
        match *self {
            true  => f(1),
            false => f(0)
        }
    }
}

impl<'a> ToCurlOptParam for &'a str {
    fn with_curl_opt_param(&self, f:|x: uintptr_t|) {
        let c_string = self.to_c_str();
        unsafe { f(mem::transmute(c_string.as_bytes().as_ptr())) };
    }
}

// NOTE: return [u8] as a *c_char will not guarantee a \0 byte at end.
//       So here I convert it to a CString.
impl<'a> ToCurlOptParam for &'a [u8] {
    fn with_curl_opt_param(&self, f:|x: uintptr_t|) {
        let c_string = self.to_c_str();
        unsafe { f(mem::transmute(c_string.as_bytes().as_ptr())) };
    }
}

impl ToCurlOptParam for Vec<String> {
    fn with_curl_opt_param(&self, f:|x: uintptr_t|) {
        f(self.iter().fold(0, |acc, item| {
                item.with_c_str(|s| {
                        unsafe { curl_slist_append(acc, s) }
                    })
            }))
    }
}

impl ToCurlOptParam for *FILE {
    fn with_curl_opt_param(&self, f:|x: uintptr_t|) {
        unsafe { f(mem::transmute(*self)) }
    }
}

impl<'r> ToCurlOptParam for |f64,f64,f64,f64|:'r -> int {
    fn with_curl_opt_param(&self, f:|x: uintptr_t|) {
        unsafe { f(mem::transmute(self)) }
    }
}


// for curl_easy_getinfo()
pub trait FromCurlInfoPtr {
    fn from_curl_info_ptr(uintptr_t) -> Self;
}

impl FromCurlInfoPtr for String {
    fn from_curl_info_ptr(ptr: uintptr_t) -> String {
        if ptr == 0 {           // dummy create :), rust use this to identify which type to use
            "".to_string()
        } else {
            unsafe {
                let p : **c_char = mem::transmute(ptr);
                // CString -> Option<&'a str> -> &'a str -> String
                CString::new(*p, false).as_str().unwrap().to_str()
            }
        }
    }
}

impl FromCurlInfoPtr for int {
    fn from_curl_info_ptr(ptr: uintptr_t) -> int {
        if ptr == 0 {           // dummy create :), rust use this to identify which type to use
            0
        } else {
            unsafe {
                let p : *c_long = mem::transmute(ptr);
                *p as int
            }
        }
    }
}

impl FromCurlInfoPtr for f64 {
    fn from_curl_info_ptr(ptr: uintptr_t) -> f64 {
        if ptr == 0 {           // dummy create :), rust use this to identify which type to use
            0f64
        } else {
            unsafe {
                let p : *c_double = mem::transmute(ptr);
                *p as f64
            }
        }
    }
}

impl FromCurlInfoPtr for Vec<String> {
    fn from_curl_info_ptr(ptr: uintptr_t) -> Vec<String> {
        if ptr == 0 {           // dummy create :), rust use this to identify which type to use
            Vec::new()
        } else {
            // unsafe {
            // let head: *SList = mem::transmute(ptr);
            // let mut p: SList = head;
            // let mut ret : ~[String] = ~[];
            // while p != 0 {
            //     ret.append(CString::new(p.data, false).as_str().to_str());
            //     p = p.next;
            // }
            // ret
            // TODO: implement, free slist
            vec!("DUMMY-INFO-SLIST-RETURN".to_string())
        }
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
    pub fn escape(&self, url: &str) -> String {
        url.with_c_str(|c_buf| {
                unsafe {
                    let ret = curl_easy_escape(self.handle, c_buf, url.len() as c_int);
                    // FIXME: owns c buffer, and free it, or not owns c buffer, manually call curl_free
                    CString::new(ret, true).as_str().unwrap().to_str()
                }
            })
    }

    pub fn init() -> Curl {
        Curl { handle: unsafe { curl_easy_init() } }
    }

    /// empty fn, use Drop trait instead
    pub fn cleanup(&self) {
        // unsafe { curl_easy_cleanup(self.handle) }
    }

    pub fn duphandle(&self) -> Curl {
        Curl { handle: unsafe { curl_easy_duphandle(self.handle) } }
    }

    pub fn getinfo<T: FromCurlInfoPtr>(&self, option: c_int) -> Option<T> {
        //let inf : T = FromCurlInfoPtr::from_curl_info_ptr(0 as uintptr_t);
        //let p = inf.new_ptr();
        let mut t: T = unsafe { mem::zeroed() };
        let p: *mut T = &mut t;
        let ret = unsafe { curl_easy_getinfo(self.handle, option, p as *mut c_void) };
        if ret == 0 {           // OK
            let val : T = unsafe { FromCurlInfoPtr::from_curl_info_ptr(mem::transmute(p)) };
            Some(val)
        } else {
            debug!("!!!! fail getinfo() ret={}", ret);
            None
        }
    }

    pub fn perform(&self) -> int {
        let ret = unsafe { curl_easy_perform(self.handle) };
        ret as int
    }

    pub fn setopt<T: ToCurlOptParam>(&self, option: c_int, param: T) -> int {
        match option {
            opt::PROGRESSFUNCTION =>
                unsafe { curl_easy_setopt(self.handle, option, mem::transmute(c_curl_cb_progress_fn)) as int },
            opt::WRITEFUNCTION =>
                unsafe { curl_easy_setopt(self.handle, option, mem::transmute(c_curl_cb_write_fn)) as int },
            opt::READFUNCTION =>
                unsafe { curl_easy_setopt(self.handle, option, mem::transmute(c_curl_cb_read_fn)) as int },
            opt::HEADERFUNCTION =>
                unsafe { curl_easy_setopt(self.handle, option, mem::transmute(c_curl_cb_header_fn)) as int },
            _ =>
                unsafe {
                    let mut res = 0;
                    param.with_curl_opt_param(|param| {
                        res = curl_easy_setopt(self.handle, option, param) as int;
                    });
                    res
                }
        }
    }

    pub fn reset(&self) {
        unsafe { curl_easy_reset(self.handle) }
    }

    pub fn unescape(&self, url: &str) -> String {
        let outlen: c_int = 0;
        url.with_c_str(|c_buf| {
                unsafe {
                    let ret = curl_easy_unescape(self.handle, c_buf, url.len() as c_int, &outlen);
                    let unescaped_url = str::raw::from_buf_len(ret as *u8, outlen as uint);
                    curl_free(ret);
                    unescaped_url
                }
            })
    }
}

pub fn strerror(code: int) -> String {
    unsafe {
        let cver = CString::new(curl_easy_strerror(code as c_int), false);
        cver.as_str().unwrap().to_str()
    }
}


// Callback

#[allow(unused_variable)]
pub extern "C" fn c_curl_cb_progress_fn(user_data: uintptr_t, dltotal: c_double,  dlnow: c_double,
                                        ultotal: c_double, ulnow: c_double) -> c_int {
    print!("\x08\x08\x08\x08\x08\x08\x08now: = {}%\r", dlnow/dltotal*100f64);
    if dlnow > 8000f64 {
        1
    } else {
        0
    }
}

// size_t function( char *ptr, size_t size, size_t nmemb, void *userdata);
#[allow(unused_variable)]
pub extern "C" fn c_curl_cb_write_fn(p: *c_char, size: size_t, nmemb: size_t, user_data: uintptr_t) -> size_t {
    size * nmemb
}

// size_t function( void *ptr, size_t size, size_t nmemb, void *userdata);
#[allow(unused_variable)]
pub extern "C" fn c_curl_cb_read_fn(p: *c_char, size: size_t, nmemb: size_t, user_data: uintptr_t) -> size_t {
    0
}

// size_t function( void *ptr, size_t size, size_t nmemb, void *userdata);
#[allow(unused_variable)]
pub extern "C" fn c_curl_cb_header_fn(p: *c_char, size: size_t, nmemb: size_t, user_data: uintptr_t) -> size_t {
    0
}


pub trait ToCurlProgressFn {
    fn to_curl_opt_param(&self) -> uintptr_t;
}
