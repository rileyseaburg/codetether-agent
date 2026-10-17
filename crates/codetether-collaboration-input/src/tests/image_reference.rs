//! Invalid image URLs fail clearly instead of becoming text-only input.

#[test]
fn rejects_invalid_or_unsupported_remote_references() {
    for value in [
        "",
        "not a URL",
        "/tmp/image.png",
        "//example.invalid/image.png",
        "https://",
        "http://[invalid]/image.png",
        "https:example.invalid/image.png",
        "https:///example.invalid/image.png",
        "https:example.invalid/path://image.png",
        "https://example.invalid/with space.png",
        "https://example.invalid/\nimage.png",
        "https://example.invalid\\image.png",
        "ftp://example.invalid/image.png",
        "file:///tmp/image.png",
        "javascript:alert(1)",
        "blob:https://example.invalid/id",
    ] {
        let error = super::remote(value).err().expect("invalid URL must fail");
        assert!(error.to_string().contains("image_url"));
    }
}

#[test]
fn unsupported_scheme_error_names_allowed_schemes() {
    let error = super::remote("ftp://example.invalid/image.png")
        .err()
        .unwrap();
    assert!(error.to_string().contains("http, https"));
}
