//! Preserved token-generation regression.
use super::*;

#[test]
fn matches_botocore_golden_vector() {
    let now = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
    let token = generate_at(&test_creds(), "us-east-1", 43200, now);
    let expected = "bedrock-api-key-YmVkcm9jay5hbWF6b25hd3MuY29tLz9BY3Rpb249Q2FsbFdpdGhCZWFyZXJUb2tlbiZYLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFJT1NGT0ROTjdFWEFNUExFJTJGMjAyNDAxMTUlMkZ1cy1lYXN0LTElMkZiZWRyb2NrJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDAxMTVUMTIwMDAwWiZYLUFtei1FeHBpcmVzPTQzMjAwJlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCZYLUFtei1TaWduYXR1cmU9NTZhNGFlMWY2OTkyYTdjZjhkNjEwYmQyODg5NTIyYTRiOTZhZWU1YjQyMDM2MzEzYTU4YTM4OGI1NGMxYzcxYyZWZXJzaW9uPTE=";
    assert_eq!(token, expected);
}
