//! Tests for refreshed-key secret construction.

use super::super::exported::Exported;
use super::build;
use crate::provider::bedrock::AwsCredentials;
use crate::secrets::ProviderSecrets;
use chrono::{TimeZone, Utc};
use serde_json::json;

fn prior() -> ProviderSecrets {
    let mut s = ProviderSecrets {
        api_key: Some("old-key".into()),
        ..ProviderSecrets::default()
    };
    s.extra.insert("sso_refresh_token".into(), json!("old-rt"));
    s.extra.insert("sso_client_id".into(), json!("cid"));
    s
}

fn exported(rotated: Option<&str>) -> Exported {
    Exported {
        creds: AwsCredentials {
            access_key_id: "AKIA".into(),
            secret_access_key: "s".into(),
            session_token: None,
        },
        expiration: Some(Utc.with_ymd_and_hms(2030, 1, 1, 0, 0, 0).unwrap()),
        rotated_refresh_token: rotated.map(str::to_string),
    }
}

#[test]
fn keeps_prior_refresh_token_when_not_rotated() {
    let s = build(&prior(), "new-key", Utc::now(), &exported(None));
    assert_eq!(s.api_key.as_deref(), Some("new-key"));
    assert_eq!(s.extra["sso_refresh_token"], json!("old-rt"));
    assert_eq!(s.extra["sso_client_id"], json!("cid"));
    assert!(s.extra.contains_key("api_key_expires_at"));
}

#[test]
fn persists_rotated_refresh_token() {
    let s = build(&prior(), "new-key", Utc::now(), &exported(Some("new-rt")));
    assert_eq!(s.extra["sso_refresh_token"], json!("new-rt"));
    assert_eq!(s.extra["sso_client_id"], json!("cid"));
}
