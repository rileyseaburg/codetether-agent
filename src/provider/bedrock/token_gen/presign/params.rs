//! Query-parameter construction and URL encoding for the presigned token.

use super::super::super::auth::AwsCredentials;

/// Build the ordered, URL-encoded SigV4 query parameters (no signature).
pub(super) fn encoded_params(
    creds: &AwsCredentials,
    scope: &str,
    amz_date: &str,
    expires_secs: u64,
) -> Vec<(String, String)> {
    let mut params: Vec<(String, String)> = vec![
        ("Action".into(), "CallWithBearerToken".into()),
        ("X-Amz-Algorithm".into(), "AWS4-HMAC-SHA256".into()),
        (
            "X-Amz-Credential".into(),
            format!("{}/{scope}", creds.access_key_id),
        ),
        ("X-Amz-Date".into(), amz_date.to_string()),
        ("X-Amz-Expires".into(), expires_secs.to_string()),
        ("X-Amz-SignedHeaders".into(), "host".into()),
    ];
    if let Some(token) = &creds.session_token {
        params.push(("X-Amz-Security-Token".into(), token.clone()));
    }
    params
        .iter()
        .map(|(k, v)| {
            (
                urlencoding::encode(k).into_owned(),
                urlencoding::encode(v).into_owned(),
            )
        })
        .collect()
}

/// Join `key=value` pairs with `&`.
pub(super) fn join(pairs: &[(String, String)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&")
}
