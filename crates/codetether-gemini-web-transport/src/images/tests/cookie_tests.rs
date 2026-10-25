use crate::images::netscape;
use reqwest::Url;

#[test]
fn netscape_scope_expiry_and_empty_values_are_preserved() {
    let source = "# Netscape file\n#HttpOnly_.google.com\tTRUE\t/\tTRUE\t0\tsession\tsynthetic\n\
gemini.google.com\tFALSE\t/app\tTRUE\t0\thost\tlocal\n\
.google.com\tTRUE\t/\tTRUE\t1\texpired\told\n\
.google.com\tTRUE\t/\tTRUE\t0\tempty\t\n";
    let jar = netscape::parse(source).unwrap();
    let header = |url| jar.header(&Url::parse(url).unwrap(), &[]);
    assert_eq!(
        header("https://gemini.google.com/app"),
        "host=local; session=synthetic; empty="
    );
    assert!(!header("https://gemini.google.com/application").contains("host="));
    assert!(header("https://work.fife.usercontent.google.com/rd-gg-dl/x").contains("session="));
    for url in [
        "https://google.com.evil.invalid/",
        "https://lh3.googleusercontent.com/",
        "http://gemini.google.com/",
    ] {
        assert!(header(url).is_empty());
    }
}

#[test]
fn malformed_exports_fail_without_echoing_values() {
    for text in [
        "secret=value",
        ".google.com\tTRUE\t/\tTRUE\t0\tx\tprivate;value",
    ] {
        let error = netscape::parse(text).err().unwrap().to_string();
        assert!(!error.contains("private"));
        assert!(!error.contains("secret"));
    }
}
