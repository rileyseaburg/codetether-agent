//! Golden-vector tests for Bedrock bearer-token generation.
//!
//! The expected values were produced by AWS's official
//! `aws-bedrock-token-generator-python` (botocore `SigV4QueryAuth`)
//! with a frozen clock of 2024-01-15T12:00:00Z.

use super::generate_at;
use crate::provider::bedrock::AwsCredentials;
use base64::Engine;
use chrono::{TimeZone, Utc};

fn test_creds() -> AwsCredentials {
    AwsCredentials {
        access_key_id: "AKIAIOSFODNN7EXAMPLE".into(),
        secret_access_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".into(),
        session_token: None,
    }
}

#[test]
fn matches_botocore_golden_vector() {
    let now = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
    let token = generate_at(&test_creds(), "us-east-1", 43200, now);
    let expected = "bedrock-api-key-YmVkcm9jay5hbWF6b25hd3MuY29tLz9BY3Rpb249Q2FsbFdpdGhCZWFyZXJUb2tlbiZYLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFJT1NGT0ROTjdFWEFNUExFJTJGMjAyNDAxMTUlMkZ1cy13ZXN0LTIlMkZiZWRyb2NrJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDAxMTVUMTIwMDAwWiZYLUFtei1FeHBpcmVzPTQzMjAwJlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCZYLUFtei1TaWduYXR1cmU9Mzc3OTMyYzY0YjUzYTE5MjcyNDliN2JlOWU1NjMyMzA0OGIwZWI0N2U1OTYzNTdiZTFiMWRiMTNmYjA3ZGRlMyZWZXJzaW9uPTE=";
    assert_eq!(token, expected);
}

#[test]
fn session_token_is_included_in_signed_query() {
    let now = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
    let mut creds = test_creds();
    creds.session_token = Some("FwoGZXIvYXdzEBc/token+chars=".into());
    let token = generate_at(&creds, "us-east-1", 3600, now);
    let raw = base64::engine::general_purpose::STANDARD
        .decode(token.strip_prefix("bedrock-api-key-").unwrap())
        .unwrap();
    let url = String::from_utf8(raw).unwrap();
    assert!(url.contains("X-Amz-Security-Token="));
    assert!(url.contains("X-Amz-Expires=3600"));
    assert!(url.ends_with("&Version=1"));
}
