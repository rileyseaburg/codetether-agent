use super::*;

#[test]
fn max_age_zero_removes_matching_cookie() {
    let mut jar = Vec::new();
    set_server_cookie(&mut jar, "sid=old; Path=/", "https://app.test/").unwrap();
    set_server_cookie(&mut jar, "sid=gone; Max-Age=0; Path=/", "https://app.test/").unwrap();

    assert_eq!(cookie_header(&jar, "https://app.test/"), "");
}

#[test]
fn expired_cookies_are_not_persisted() {
    let mut jar = Vec::new();
    set_server_cookie(&mut jar, "old=1; Expires=1; Path=/", "https://app.test/").unwrap();
    set_server_cookie(&mut jar, "live=1; Path=/", "https://app.test/").unwrap();

    assert_eq!(persistent_cookies(&jar).len(), 1);
    assert_eq!(cookie_header(&jar, "https://app.test/"), "live=1");
}
