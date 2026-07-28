//! Precomputed HTTP response buffers for static routes.

use std::sync::Arc;

use super::super::http_status::reason_phrase;

/// Cached GET and HEAD response bytes for one static resource.
#[derive(Clone)]
pub(crate) struct CachedResponse {
    get_keep: Arc<Vec<u8>>,
    get_close: Arc<Vec<u8>>,
    head_keep: Arc<Vec<u8>>,
    head_close: Arc<Vec<u8>>,
}

impl CachedResponse {
    /// Build all connection/method response variants up front.
    pub(crate) fn new(status: u16, content_type: &str, body: Vec<u8>) -> Self {
        Self {
            get_keep: response(status, content_type, &body, true, true),
            get_close: response(status, content_type, &body, false, true),
            head_keep: response(status, content_type, &body, true, false),
            head_close: response(status, content_type, &body, false, false),
        }
    }

    /// Select the prebuilt bytes for method and connection persistence.
    pub(crate) fn bytes(&self, method: &str, keep_alive: bool) -> &[u8] {
        match (method == "HEAD", keep_alive) {
            (true, true) => &self.head_keep,
            (true, false) => &self.head_close,
            (false, true) => &self.get_keep,
            (false, false) => &self.get_close,
        }
    }
}

fn response(
    status: u16,
    content_type: &str,
    body: &[u8],
    keep_alive: bool,
    include_body: bool,
) -> Arc<Vec<u8>> {
    let connection = if keep_alive { "keep-alive" } else { "close" };
    let mut bytes = format!(
        "HTTP/1.1 {} {}\r\ncontent-length: {}\r\ncontent-type: {}\r\nconnection: {}\r\n\r\n",
        status,
        reason_phrase(status),
        body.len(),
        content_type,
        connection
    )
    .into_bytes();
    if include_body {
        bytes.extend_from_slice(body);
    }
    Arc::new(bytes)
}
