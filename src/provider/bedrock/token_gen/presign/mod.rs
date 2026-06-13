//! SigV4 query-string presigning for Bedrock bearer tokens.
//!
//! Builds the presigned query for
//! `POST https://bedrock.amazonaws.com/?Action=CallWithBearerToken`,
//! mirroring botocore's `SigV4QueryAuth` so tokens match AWS's official
//! `aws-bedrock-token-generator` output byte-for-byte.

mod params;
mod sign;

use super::super::auth::AwsCredentials;
use super::super::sigv4::sha256_hex;
use chrono::{DateTime, Utc};

const HOST: &str = "bedrock.amazonaws.com";
/// SHA-256 of the empty string (presigned requests carry no body).
const EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

/// Build the full presigned query string, signature included.
pub(super) fn presigned_query(
    creds: &AwsCredentials,
    region: &str,
    expires_secs: u64,
    now: DateTime<Utc>,
) -> String {
    let datestamp = now.format("%Y%m%d").to_string();
    let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
    let scope = format!("{datestamp}/{region}/bedrock/aws4_request");
    let encoded = params::encoded_params(creds, &scope, &amz_date, expires_secs);
    let query = params::join(&encoded);
    let mut sorted = encoded;
    sorted.sort();
    let canonical_request = format!(
        "POST\n/\n{}\nhost:{HOST}\n\nhost\n{EMPTY_SHA256}",
        params::join(&sorted)
    );
    let string_to_sign = format!(
        "AWS4-HMAC-SHA256\n{amz_date}\n{scope}\n{}",
        sha256_hex(canonical_request.as_bytes())
    );
    format!(
        "{query}&X-Amz-Signature={}",
        sign::signature(creds, region, &datestamp, &string_to_sign)
    )
}
