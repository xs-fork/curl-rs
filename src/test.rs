#![license = "MIT"]


extern crate libc;
//extern crate curl;
//use super;
use super::easy;
use super::easy::Curl;
use super::info;
use libc::{tmpfile, fclose};
use super::opt;


static TEST_URL : &'static str = "http://www.baidu.com/";

#[test]
fn test_version() {
    assert!(super::version().len() > 0)
}

#[test]
fn test_easy_init() {
    let c = Curl::new();
    assert!(!c.is_null());
    c.cleanup()
}

#[test]
fn test_easy_perform_only() {
    let c = Curl::new();
    let ret = c.perform();
    assert!(ret == 3);
    c.cleanup();
}

#[test]
fn test_easy_strerror() {
    assert!(easy::strerror(0).as_slice() == "No error");
    assert!(easy::strerror(3).len() > 0);
}

#[test]
fn test_easy_escape() {
    let c = Curl::new();
    assert_eq!(c.escape("abcEFG").as_slice(), "abcEFG");
    assert_eq!(c.escape("&*()").as_slice(), "%26%2A%28%29");
    // c.escape("\x00fuck"));
    c.cleanup();
}

#[test]
fn test_easy_duphandle() {
    let c = Curl::new();
    assert!(!c.is_null());
    let cc = c.duphandle();
    assert!(!c.is_null());
    c.cleanup();
    cc.cleanup();
}

#[test]
fn test_easy_reset() {
    let c = Curl::new();
    c.reset();
    assert!(!c.is_null());
    c.cleanup();
}

#[test]
fn test_easy_unescape() {
    let c = Curl::new();
    assert_eq!(c.unescape("abcEFG").as_slice(), "abcEFG");
    assert_eq!(c.unescape("%26%2A%28%29").as_slice(), "&*()");
    c.cleanup();
}

#[test]
fn test_easy_setopt_url() {
    let c = Curl::new();
    assert_eq!(c.setopt(opt::URL, TEST_URL), 0);
    let ret = c.perform();
    let _ : Option<String> = c.getinfo(info::EFFECTIVE_URL);
    assert!(ret == 0 || ret == 7); // OK or cound't connect
    c.cleanup();
}

#[test]
fn test_easy_setopt() {
    let c = Curl::new();
    assert_eq!(c.setopt(opt::URL, TEST_URL), 0);
    assert_eq!(c.setopt(opt::VERBOSE, false), 0);
    let ret = c.perform();
    let _ : Option<String> = c.getinfo(info::EFFECTIVE_URL);
    assert_eq!(ret, 0);
}

#[test]
fn test_easy_setopt_bytes() {
    let c = Curl::new();
    assert_eq!(c.setopt(opt::URL, b"http://www.baidu.com/"), 0);
    assert_eq!(c.setopt(opt::VERBOSE, false), 0);
    let ret = c.perform();
    assert_eq!(ret, 0);
    c.cleanup();
}

#[test]
fn test_global_init() {
    let ret = super::global_init(super::GLOBAL_ALL);
    assert_eq!(ret, 0);
    // curl::global_cleanup()
}

#[test]
fn test_easy_setopt_slist() {
    let c = Curl::new();
    assert_eq!(c.setopt(opt::URL, "http://fledna.duapp.com/ip"), 0);
    c.setopt(opt::HTTPHEADER, vec!("X-Dummy: just a test.".to_string()));
    assert_eq!(c.setopt(opt::VERBOSE, false), 0);
    let ret = c.perform();
    assert_eq!(ret, 0);
    c.cleanup();
}

#[test]
fn test_easy_setopt_writedata() {
    let c = Curl::new();
    assert_eq!(c.setopt(opt::URL, TEST_URL), 0);

    let fp = unsafe { tmpfile() };

    c.setopt(opt::WRITEDATA, fp);
    c.setopt(opt::VERBOSE, false);
    c.perform();

    unsafe { fclose(fp) };
}


#[test]
fn test_easy_setopt_progress_function() {
    let c = Curl::new();
    assert_eq!(c.setopt(opt::URL, "http://curl.haxx.se/download/curl-7.34.0.zip"), 0);
    // let func: |f64,f64,f64,f64| -> int = |dltotal, dlnow, ultotal, ulnow| {
    //     println!("progress func test: {} {} {} {}", dltotal, dlnow, ultotal, ulnow);
    //     0
    // };
    c.setopt(opt::NOPROGRESS, false);
    c.setopt(opt::WRITEFUNCTION, 0i);
    let ret = c.setopt(opt::PROGRESSFUNCTION, 0i);
    assert_eq!(ret, 0);
    assert_eq!(c.perform(), 42);
}

#[test]
fn test_easy_getinfo() {
    let c = Curl::new();
    c.setopt(opt::URL, TEST_URL);
    c.setopt(opt::WRITEFUNCTION, 0i);
    c.perform();

    let mut val : Option<int> = c.getinfo(info::RESPONSE_CODE);
    assert_eq!(val.unwrap(), 200);
    val = c.getinfo(info::REQUEST_SIZE);
    assert!(val.unwrap() > 0);
    let tt : Option<f64> = c.getinfo(info::TOTAL_TIME);
    assert!(tt.unwrap() > 0f64);
}
