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
    let expected = "bedrock-api-key-YmVkcm9jay5hbWF6b25hd3MuY29tLz9BY3Rpb249Q2FsbFdpdGhCZWFyZXJUb2tlbiZYLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFJT1NGT0ROTjdFWEFNUExFJTJGMjAyNDAxMTUlMkZ1cy1lYXN0LTElMkZiZWRyb2NrJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDAxMTVUMTIwMDAwWiZYLUFtei1FeHBpcmVzPTQzMjAwJlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCZYLUFtei1TaWduYXR1cmU9NTZhNGFlMWY2OTkyYTdjZjhkNjEwYmQyODg5NTIyYTRiOTZhZWU1YjQyMDM2MzEzYTU4YTM4OGI1NGMxYzcxYyZWZXJzaW9uPTE=";
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

#[test]
fn session_token_matches_botocore_golden_vector() {
    // Golden vector from AWS's official aws-bedrock-token-generator
    // (botocore SigV4QueryAuth) with a session token, frozen at
    // 2024-01-15T12:00:00Z, region us-east-1, expires 3600.
    let now = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
    let mut creds = test_creds();
    creds.session_token = Some("FwoGZXIvYXdzEBc/token+chars=".into());
    let token = generate_at(&creds, "us-east-1", 3600, now);
    let expected = "bedrock-api-key-YmVkcm9jay5hbWF6b25hd3MuY29tLz9BY3Rpb249Q2FsbFdpdGhCZWFyZXJUb2tlbiZYLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFJT1NGT0ROTjdFWEFNUExFJTJGMjAyNDAxMTUlMkZ1cy1lYXN0LTElMkZiZWRyb2NrJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDAxMTVUMTIwMDAwWiZYLUFtei1FeHBpcmVzPTM2MDAmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JlgtQW16LVNlY3VyaXR5LVRva2VuPUZ3b0daWEl2WVhkekVCYyUyRnRva2VuJTJCY2hhcnMlM0QmWC1BbXotU2lnbmF0dXJlPTM0ZTc1NGJlNTlmOTBiODNlOTNlNjUwMzNlZmQxYTdjNGI5NmMyMDY2NDc1Y2UwN2VlOWNlY2M4MzY2MWIyYmYmVmVyc2lvbj0x";
    assert_eq!(token, expected);
}
