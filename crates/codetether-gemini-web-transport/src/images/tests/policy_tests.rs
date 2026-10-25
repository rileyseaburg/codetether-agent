use crate::images::policy::{image_url, rendition};

#[test]
fn only_observed_https_image_paths_are_allowed() {
    for url in [
        "https://lh3.googleusercontent.com/gg-dl/test",
        "https://lh3.googleusercontent.com/rd-gg-dl/test",
        "https://work.fife.usercontent.google.com/rd-gg-dl/test",
    ] {
        assert!(image_url(url).is_ok());
    }
    for url in [
        "http://lh3.googleusercontent.com/gg-dl/test",
        "https://lh3.googleusercontent.com.evil.invalid/gg-dl/test",
        "https://evil.invalid/gg-dl/test",
        "https://user:secret@lh3.googleusercontent.com/gg-dl/test",
        "https://lh3.googleusercontent.com:8443/gg-dl/test",
        "https://lh3.googleusercontent.com/gg-dl/../account",
        "https://lh3.googleusercontent.com/gg-dl/test#fragment",
        "https://work.fife.usercontent.google.com/gg-dl/test",
    ] {
        let error = image_url(url).err().unwrap().to_string();
        assert_eq!(error, "image_url_disallowed");
        assert!(!error.contains("secret"));
    }
}

#[test]
fn rendition_is_idempotent_and_preserves_other_query_parameters() {
    let base = "https://lh3.googleusercontent.com/gg-dl/test=s512-rj?token=test&alr=no";
    let url = rendition(base).unwrap();
    assert_eq!(url.path(), "/gg-dl/test=s1024-rj");
    assert!(url.query_pairs().any(|(k, v)| k == "token" && v == "test"));
    assert_eq!(url.query_pairs().filter(|(k, _)| k == "alr").count(), 1);
    assert_eq!(url, rendition(url.as_str()).unwrap());
}
