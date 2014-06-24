#[license = "MIT"];

extern crate libc;
extern crate curl;

use libc::{fopen, fclose};
use std::c_str;
use std::mem;

static TEST_URL : &'static str = "http://www.baidu.com/";

#[test]
fn test_version() {
    assert!(curl::version().len() > 0)
}

#[test]
fn test_easy_init() {
    let c = curl::easy::Curl::init();
    assert!(!c.is_null());
    c.cleanup()
}

#[test]
fn test_easy_perform_only() {
    let c = curl::easy::Curl::init();
    let ret = c.perform();
    assert!(ret == 3);
    c.cleanup();
}

#[test]
fn test_easy_strerror() {
    assert!(curl::easy::strerror(0).as_slice() == "No error");
    assert!(curl::easy::strerror(3).len() > 0);
}

#[test]
fn test_easy_escape() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.escape("abcEFG").as_slice(), "abcEFG");
    assert_eq!(c.escape("&*()").as_slice(), "%26%2A%28%29");
    // c.escape("\x00fuck"));
    c.cleanup();
}

#[test]
fn test_easy_duphandle() {
    let c = curl::easy::Curl::init();
    assert!(!c.is_null());
    let cc = c.duphandle();
    assert!(!c.is_null());
    c.cleanup();
    cc.cleanup();
}

#[test]
fn test_easy_reset() {
    let c = curl::easy::Curl::init();
    c.reset();
    assert!(!c.is_null());
    c.cleanup();
}

#[test]
fn test_easy_unescape() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.unescape("abcEFG").as_slice(), "abcEFG");
    assert_eq!(c.unescape("%26%2A%28%29").as_slice(), "&*()");
    c.cleanup();
}

#[test]
fn test_easy_setopt_URL() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.setopt(curl::opt::URL, TEST_URL), 0);
    let ret = c.perform();
    let eurl : Option<String> = c.getinfo(curl::info::EFFECTIVE_URL);
    assert!(ret == 0 || ret == 7); // OK or cound't connect
    c.cleanup();
}

#[test]
fn test_easy_setopt() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.setopt(curl::opt::URL, TEST_URL), 0);
    assert_eq!(c.setopt(curl::opt::VERBOSE, false), 0);
    let ret = c.perform();
    let eurl : Option<String> = c.getinfo(curl::info::EFFECTIVE_URL);
    assert_eq!(ret, 0);
}

#[test]
fn test_easy_setopt_bytes() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.setopt(curl::opt::URL, bytes!("http://www.baidu.com/")), 0);
    assert_eq!(c.setopt(curl::opt::VERBOSE, false), 0);
    let ret = c.perform();
    assert_eq!(ret, 0);
    c.cleanup();
}

#[test]
fn test_global_init() {
    let ret = curl::global_init(curl::GLOBAL_ALL);
    assert_eq!(ret, 0);
    // curl::global_cleanup()
}

#[test]
fn test_easy_setopt_slist() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.setopt(curl::opt::URL, "http://fledna.duapp.com/ip"), 0);
    c.setopt(curl::opt::HTTPHEADER, vec!("X-Dummy: just a test.".to_string()));
    assert_eq!(c.setopt(curl::opt::VERBOSE, false), 0);
    let ret = c.perform();
    assert_eq!(ret, 0);
    c.cleanup();
}

#[test]
fn test_easy_setopt_writedata() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.setopt(curl::opt::URL, TEST_URL), 0);
    let fp = "/tmp/test.out".to_c_str().with_ref(|fname| {
            "w".to_c_str().with_ref(|mode| {
                    unsafe { fopen(fname, mode) }
                })
                });
    c.setopt(curl::opt::WRITEDATA, fp);
    c.setopt(curl::opt::VERBOSE, false);
    c.perform();
    unsafe { fclose(fp) };
}


#[test]
fn test_easy_setopt_progress_function() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.setopt(curl::opt::URL, "http://curl.haxx.se/download/curl-7.34.0.zip"), 0);
    // let func: |f64,f64,f64,f64| -> int = |dltotal, dlnow, ultotal, ulnow| {
    //     println!("progress func test: {} {} {} {}", dltotal, dlnow, ultotal, ulnow);
    //     0
    // };
    c.setopt(curl::opt::NOPROGRESS, false);
    c.setopt(curl::opt::WRITEFUNCTION, 0);
    let ret = c.setopt(curl::opt::PROGRESSFUNCTION, 0);
    assert_eq!(ret, 0);
    assert_eq!(c.perform(), 42);
}

#[test]
fn test_easy_getinfo() {
    let c = curl::easy::Curl::init();
    c.setopt(curl::opt::URL, TEST_URL);
    c.setopt(curl::opt::WRITEFUNCTION, 0);
    c.perform();

    let mut val : Option<int> = c.getinfo(curl::info::RESPONSE_CODE);
    assert_eq!(val.unwrap(), 200);
    val = c.getinfo(curl::info::REQUEST_SIZE);
    assert!(val.unwrap() > 0);
    let tt : Option<f64> = c.getinfo(curl::info::TOTAL_TIME);
    assert!(tt.unwrap() > 0f64);
}
