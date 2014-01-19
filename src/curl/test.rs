#[license = "MIT"];


extern mod curl;

#[test]
fn test_version() {
    assert!(curl::version().len() > 0)
}
