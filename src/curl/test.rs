#[license = "MIT"];


extern mod curl;

#[test]
fn test_version() {
    assert!(curl::version().len() > 0)
}

#[test]
fn test_easy_init() {
    let h = curl::easy::init();
    assert!(!curl::easy::init().is_null());
    curl::easy::cleanup(h)
}

#[test]
fn test_easy_curl_init() {
    let c = curl::easy::Curl::init();
    assert!(!c.is_null());
    c.cleanup()
}

#[test]
fn test_easy_curl_perform() {
    let c = curl::easy::Curl::init();
    let ret = c.perform();
    assert!(ret == 3);
    c.cleanup();
}

#[test]
fn test_easy_strerror() {
    assert!(curl::easy::strerror(0) == ~"No error");
    assert!(curl::easy::strerror(3).len() > 0);
}

#[test]
fn test_easy_escape() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.escape("abcEFG"), ~"abcEFG");
    assert_eq!(c.escape("&*()"), ~"%26%2A%28%29");
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
    assert_eq!(c.unescape("abcEFG"), ~"abcEFG");
    assert_eq!(c.unescape("%26%2A%28%29"), ~"&*()");
    c.cleanup();
}
