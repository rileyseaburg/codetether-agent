//! Endpoint scheme and trailing slash normalization.

/// Normalize endpoint URL (ensure protocol, remove trailing slash)
pub(super) fn normalize_endpoint(endpoint: &str, secure: bool) -> String {
    let endpoint = endpoint.trim_end_matches('/');

    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        endpoint.to_string()
    } else if secure {
        format!("https://{endpoint}")
    } else {
        format!("http://{endpoint}")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_endpoint() {
        assert_eq!(
            normalize_endpoint("localhost:9000", false),
            "http://localhost:9000"
        );
        assert_eq!(
            normalize_endpoint("localhost:9000", true),
            "https://localhost:9000"
        );
        assert_eq!(
            normalize_endpoint("http://localhost:9000/", false),
            "http://localhost:9000"
        );
        assert_eq!(
            normalize_endpoint("https://minio.example.com/", true),
            "https://minio.example.com"
        );
    }
}
