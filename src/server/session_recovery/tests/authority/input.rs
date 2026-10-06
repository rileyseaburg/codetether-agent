//! Invalid authority inputs must fail before any network request is possible.
use super::{HeaderMap, headers, http_error, workspace};

#[tokio::test]
async fn rejects_missing_or_malformed_credentials() {
    for value in [None, Some("Basic test"), Some("Bearer "), Some("Bearer   ")] {
        let mut headers = HeaderMap::new();
        if let Some(value) = value {
            headers.insert("authorization", value.parse().unwrap());
        }
        let error = workspace("not-a-url", "workspace", &headers)
            .await
            .unwrap_err();
        assert_eq!(http_error(error).0.as_u16(), 401);
    }
}

#[tokio::test]
async fn rejects_dot_ids_and_invalid_authorities() {
    for id in ["", ".", ".."] {
        let error = workspace("not-a-url", id, &headers()).await.unwrap_err();
        assert_eq!(http_error(error).0.as_u16(), 403);
    }
    for server in [
        "not-a-url",
        "file:///tmp",
        "https://user:password@localhost",
    ] {
        let error = workspace(server, "workspace", &headers())
            .await
            .unwrap_err();
        assert_eq!(http_error(error).0.as_u16(), 503);
    }
}
