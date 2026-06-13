//! SigV4 signing-key derivation for the `bedrock` service.

use super::super::super::auth::AwsCredentials;
use super::super::super::sigv4::hmac_sha256;

/// Standard SigV4 signing-key derivation, returning the hex signature.
pub(super) fn signature(
    creds: &AwsCredentials,
    region: &str,
    datestamp: &str,
    string_to_sign: &str,
) -> String {
    let k_date = hmac_sha256(
        format!("AWS4{}", creds.secret_access_key).as_bytes(),
        datestamp.as_bytes(),
    );
    let k_region = hmac_sha256(&k_date, region.as_bytes());
    let k_service = hmac_sha256(&k_region, b"bedrock");
    let k_signing = hmac_sha256(&k_service, b"aws4_request");
    hex::encode(hmac_sha256(&k_signing, string_to_sign.as_bytes()))
}
