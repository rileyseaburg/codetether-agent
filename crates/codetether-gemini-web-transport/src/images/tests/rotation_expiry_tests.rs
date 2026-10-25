//! Cookie expiry formats observed in Google responses, without live credentials.
use crate::images::rotation;
use std::time::UNIX_EPOCH;

#[test]
fn google_cookie_expiry_accepts_hyphenated_four_digit_year() {
    let header = "__Secure-1PSIDTS=fixture; Domain=.google.com; Path=/; Secure; \
        Expires=Sun, 24-Oct-2027 04:20:54 GMT";
    let cookie = rotation::from_header(header).expect("Google cookie date format");
    let expected = httpdate::parse_http_date("Sun, 24 Oct 2027 04:20:54 GMT")
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    assert_eq!(cookie.expires, expected);
    let standard = rotation::from_header(&header.replace("24-Oct-2027", "24 Oct 2027"))
        .expect("standard HTTP date remains supported");
    assert_eq!(cookie.expires, standard.expires);
}

#[test]
fn cookie_expiry_stays_strict_and_max_age_takes_precedence() {
    let prefix = "__Secure-1PSIDTS=fixture; Domain=.google.com; Path=/; Secure";
    for date in ["invalid", "Sun, 99-Oct-2027 04:20:54 GMT", ""] {
        assert!(rotation::from_header(&format!("{prefix}; Expires={date}")).is_none());
    }
    let expired = rotation::from_header(&format!(
        "{prefix}; Expires=Sun, 24-Oct-2027 04:20:54 GMT; Max-Age=0"
    ))
    .unwrap();
    assert_eq!(expired.expires, 0);
    assert!(rotation::from_header(&format!("{prefix}; Max-Age=invalid")).is_none());
}
