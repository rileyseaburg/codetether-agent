//! Tests for AWS SSO metadata extraction used by Bedrock auth.

use super::sso_cache::{SsoTokenCache, matches_profile};
use super::sso_profile_meta::{SsoProfileMeta, from_config_text};

#[test]
fn resolves_session_backed_profile() {
    let text = "[profile dev]\nsso_session = corp\nsso_account_id = 123\nsso_role_name = Power\n\
                [sso-session corp]\nsso_start_url = https://example.awsapps.com/start\n\
                sso_region = us-east-1\n";
    let meta = from_config_text("dev", text).expect("sso profile");
    assert_eq!(meta.start_url, "https://example.awsapps.com/start");
    assert_eq!(meta.account_id.as_deref(), Some("123"));
}

#[test]
fn matches_start_url_without_trailing_slash() {
    let cache = SsoTokenCache {
        start_url: Some("https://example.awsapps.com/start/".into()),
        refresh_token: Some("refresh".into()),
        access_token: None,
        expires_at: None,
        client_id: None,
        client_secret: None,
        registration_expires_at: None,
    };
    let meta = SsoProfileMeta {
        profile: "dev".into(),
        start_url: "https://example.awsapps.com/start".into(),
        sso_region: "us-east-1".into(),
        account_id: None,
        role_name: None,
    };
    assert!(matches_profile(&cache, &meta));
}
