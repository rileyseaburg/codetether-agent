//! Resource response metadata and body.
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct ResourceTiming {
    pub dns: Duration,
    pub connect: Duration,
    pub request: Duration,
    pub response: Duration,
}

#[derive(Debug, Clone)]
pub struct ResourceResponse {
    pub url: String,
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub content_type: Option<String>,
    pub content_length: Option<u64>,
    pub timing: ResourceTiming,
}

impl ResourceResponse {
    pub fn new(
        url: impl Into<String>,
        status: u16,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    ) -> Self {
        let content_type = headers.get("content-type").cloned();
        let content_length = headers
            .get("content-length")
            .and_then(|v| v.parse().ok())
            .or(Some(body.len() as u64));
        Self {
            url: url.into(),
            status,
            headers,
            body,
            content_type,
            content_length,
            timing: ResourceTiming::default(),
        }
    }
}
