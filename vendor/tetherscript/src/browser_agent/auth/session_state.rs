//! Full auth session state snapshot.

use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AuthSession {
    pub cookies: Vec<CookieSnapshot>,
    pub localStorage: HashMap<String, String>,
    pub sessionStorage: HashMap<String, String>,
    pub indexeddb_summary: Vec<String>,
    pub auth_headers: Vec<(String, String)>,
    pub token_payloads: Vec<TokenClaim>,
    pub redirects: Vec<RedirectRecord>,
    pub timing: AuthTiming,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CookieSnapshot {
    pub name: String, pub value: String,
    pub domain: String, pub path: String,
    pub secure: bool, pub httponly: bool,
    pub samesite: Option<String>, pub expires: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TokenClaim {
    pub kind: String,
    pub issuer: Option<String>,
    pub subject: Option<String>,
    pub expires_at: Option<i64>,
    pub raw_header: String,
    pub raw_payload: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RedirectRecord {
    pub from_url: String, pub to_url: String,
    pub status_code: u16, pub headers: Vec<(String, String)>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AuthTiming {
    pub dns_ms: u64, pub connect_ms: u64,
    pub tls_ms: u64, pub ttfb_ms: u64, pub total_ms: u64,
}

impl AuthSession {
    pub fn new() -> Self { Self::default() }
    pub fn expired_tokens(&self, now_unix: i64) -> Vec<&TokenClaim> {
        self.token_payloads.iter().filter(|t| t.expires_at.map_or(false, |e| e <= now_unix)).collect()
    }
}
