//! Captured network exchange model.

use super::cors_debug::CorsAnalysis;

pub type Headers = Vec<(String, String)>;

#[derive(Clone, Debug, PartialEq)]
pub struct CapturedExchange {
    pub id: u64,
    pub request: CapturedRequest,
    pub response: Option<CapturedResponse>,
    pub timing: ExchangeTiming,
    pub cors: Option<CorsAnalysis>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapturedRequest {
    pub method: String, pub url: String,
    pub headers: Headers, pub body: Vec<u8>,
    pub initiator: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapturedResponse {
    pub status: u16, pub status_text: String,
    pub headers: Headers, pub body: Vec<u8>,
    pub encoding: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExchangeTiming {
    pub start_ms: f64, pub dns_ms: f64, pub connect_ms: f64,
    pub tls_ms: f64, pub ttfb_ms: f64, pub download_ms: f64, pub total_ms: f64,
}

/// Look up a header value by case-insensitive name (last wins).
pub fn header<'a>(headers: &'a Headers, name: &str) -> Option<&'a str> {
    headers.iter().rev().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.as_str())
}

impl CapturedExchange {
    pub fn new(id: u64, method: &str, url: &str) -> Self {
        Self { id, request: CapturedRequest { method: method.into(), url: url.into(),
            headers: vec![], body: vec![], initiator: None },
            response: None, timing: ExchangeTiming::default(), cors: None }
    }
    pub fn with_response(mut self, status: u16, headers: Headers, body: Vec<u8>) -> Self {
        self.response = Some(CapturedResponse { status, status_text: status.to_string(),
            headers, body, encoding: None }); self
    }
}
