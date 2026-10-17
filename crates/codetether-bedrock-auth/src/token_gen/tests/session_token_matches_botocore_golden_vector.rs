//! Preserved token-generation regression.
use super::*;

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
