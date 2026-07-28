//! Deterministic mock server from captured exchanges.

use super::capture::{CapturedExchange, CapturedResponse, header};

#[derive(Clone, Debug)]
pub struct MockServer { exchanges: Vec<CapturedExchange>, strict_body: bool }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockRequest { pub method: String, pub url: String, pub headers: Vec<(String, String)>, pub body: Vec<u8> }

impl MockServer {
    pub fn new(exchanges: Vec<CapturedExchange>) -> Self { Self { exchanges, strict_body: false } }
    pub fn strict_body(mut self, yes: bool) -> Self { self.strict_body = yes; self }
    pub fn respond(&self, req: &MockRequest) -> Option<CapturedResponse> {
        self.exchanges.iter().find(|e| self.matches(e, req)).and_then(|e| e.response.clone())
    }
    pub fn respond_or_404(&self, req: &MockRequest) -> CapturedResponse {
        self.respond(req).unwrap_or(CapturedResponse {
            status: 404, status_text: "Not Found".into(),
            headers: vec![("content-type".into(), "text/plain".into())],
            body: b"mock miss".to_vec(), encoding: None,
        })
    }
    fn matches(&self, e: &CapturedExchange, r: &MockRequest) -> bool {
        e.request.method == r.method && e.request.url == r.url
            && (!self.strict_body || e.request.body == r.body)
    }
}
