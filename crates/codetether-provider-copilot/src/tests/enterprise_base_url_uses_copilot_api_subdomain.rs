//! Existing regression test.
use super::*;

#[test]
fn enterprise_base_url_uses_copilot_api_subdomain() {
    let url = enterprise_base_url("https://company.ghe.com/");
    assert_eq!(url, "https://copilot-api.company.ghe.com");
}
