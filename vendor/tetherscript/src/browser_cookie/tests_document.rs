use super::*;

#[test]
fn document_cookie_hides_httponly_and_keeps_attributes() {
    let mut jar = Vec::new();
    set_server_cookie(
        &mut jar,
        "secret=1; HttpOnly; Path=/app",
        "https://app.test/app/page",
    )
    .unwrap();
    apply_document_cookies(
        &mut jar,
        vec!["visible=1; Path=/app; HttpOnly; SameSite=Strict".into()],
        "https://app.test/app/page",
    );
    apply_document_cookies(
        &mut jar,
        vec!["secret=2; Path=/app".into()],
        "https://app.test/app/page",
    );

    assert_eq!(
        document_cookie_pairs(&jar, "https://app.test/app/page"),
        vec![("visible".into(), "1".into())]
    );
    assert!(
        !jar.iter()
            .find(|cookie| cookie.name == "visible")
            .unwrap()
            .http_only
    );
    assert_eq!(
        request_cookie_header(&jar, "https://app.test/app/api", "https://app.test/"),
        "secret=1; visible=1"
    );
    assert_eq!(
        request_cookie_header(&jar, "https://app.test/app/api", "https://other.test/"),
        ""
    );
}

#[test]
fn document_cookie_rejects_foreign_domain() {
    let mut jar = Vec::new();
    apply_document_cookies(
        &mut jar,
        vec!["sid=1; Domain=other.test; Path=/".into()],
        "https://app.test/",
    );

    assert!(jar.is_empty());
}
