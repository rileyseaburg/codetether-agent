//! Canonical signed header and request construction.
use super::*;
pub(super) fn canonical_request(
    method: &str,
    canonical_uri: &str,
    canonical_querystring: &str,
    payload_hash: &str,
    host: &str,
    amz_date: &str,
    creds: &AwsCredentials,
) -> (String, String) {
    let mut headers_map: Vec<(&str, String)> = vec![
        ("content-type", "application/json".to_string()),
        ("host", host.to_owned()),
        ("x-amz-content-sha256", payload_hash.to_owned()),
        ("x-amz-date", amz_date.to_owned()),
    ];
    if let Some(token) = &creds.session_token {
        headers_map.push(("x-amz-security-token", token.clone()));
    }
    headers_map.sort_by_key(|(k, _)| *k);

    let canonical_headers: String = headers_map
        .iter()
        .map(|(k, v)| format!("{k}:{v}"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let signed_headers: String = headers_map
        .iter()
        .map(|(k, _)| *k)
        .collect::<Vec<_>>()
        .join(";");

    let canonical_request = format!(
        "{method}\n{canonical_uri}\n{canonical_querystring}\n{canonical_headers}\n{signed_headers}\n{payload_hash}"
    );
    (canonical_request, signed_headers)
}
