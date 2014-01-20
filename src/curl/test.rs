#[license = "MIT"];


extern mod curl;

#[test]
fn test_version() {
    assert!(curl::version().len() > 0)
}

#[test]
fn test_easy_init() {
    let h = curl::easy::init();
    println!("h={}", h.handle);
    //assert!(&curl::easy::init().is_null())
}
