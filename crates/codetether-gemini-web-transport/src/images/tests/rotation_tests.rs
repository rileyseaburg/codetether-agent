use crate::images::{cookies::now, netscape, rotation};
use reqwest::Url;
#[test]
fn only_scoped_secure_rotation_cookies_are_accepted() {
    let good = "__Secure-1PSIDTS=new; Domain=.google.com; Path=/; Secure; Max-Age=3600";
    let update = rotation::from_header(good).unwrap();
    assert!(update.expires > now());
    for header in [
        good.replace(".google.com", ".evil.invalid"),
        good.replace("Secure; ", ""),
        good.replace("1PSIDTS", "OTHER"),
    ] {
        assert!(rotation::from_header(&header).is_none());
    }
    let jar = netscape::parse(".google.com\tTRUE\t/\tTRUE\t0\t__Secure-1PSIDTS\told").unwrap();
    assert_eq!(
        jar.header(&Url::parse("https://gemini.google.com").unwrap(), &[update]),
        "__Secure-1PSIDTS=new"
    );
    let expired = rotation::from_header(&good.replace("3600", "0")).unwrap();
    assert!(
        jar.header(
            &Url::parse("https://gemini.google.com").unwrap(),
            &[expired]
        )
        .is_empty()
    );
}
