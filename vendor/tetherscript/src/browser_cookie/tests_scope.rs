use super::*;

#[test]
fn domain_and_path_matching_are_browser_like() {
    let mut jar = Vec::new();
    set_server_cookie(
        &mut jar,
        "sid=root; Domain=.example.test; Path=/app",
        "https://www.example.test/app/login",
    )
    .unwrap();
    set_server_cookie(&mut jar, "host=only", "https://www.example.test/app/login").unwrap();

    assert_eq!(
        cookie_header(&jar, "https://api.example.test/app/page"),
        "sid=root"
    );
    assert_eq!(
        cookie_header(&jar, "https://www.example.test/app/login/step"),
        "sid=root; host=only"
    );
    assert_eq!(
        cookie_header(&jar, "https://www.example.test/application"),
        ""
    );
}

#[test]
fn request_header_applies_samesite_metadata() {
    let mut jar = Vec::new();
    set_server_cookie(&mut jar, "lax=1; SameSite=Lax", "https://app.test/").unwrap();
    set_server_cookie(&mut jar, "open=1; SameSite=None", "https://app.test/").unwrap();

    assert_eq!(
        request_cookie_header(&jar, "https://app.test/api", "https://other.test/"),
        "open=1"
    );
    assert_eq!(
        request_cookie_header(&jar, "https://app.test/api", "https://app.test/page"),
        "lax=1; open=1"
    );
}
