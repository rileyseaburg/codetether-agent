//! Existing regression test.
use super::*;

#[test]
fn normalize_enterprise_domain_handles_scheme_and_trailing_slash() {
    for input in [
        "https://company.ghe.com/",
        "http://company.ghe.com",
        "company.ghe.com",
    ] {
        assert_eq!(normalize_enterprise_domain(input), "company.ghe.com");
    }
}
