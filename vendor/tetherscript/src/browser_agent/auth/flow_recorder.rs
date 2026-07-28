//! Auth flow recorder for tracking complete login sequences.

use super::cookie_tracker::CookieMutation;
use super::session_state::{CookieSnapshot, RedirectRecord, TokenClaim};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthEvent {
    LoginForm { url: String, fields: Vec<String> },
    Request { method: String, url: String, headers: Vec<(String, String)> },
    Response { url: String, status: u16, headers: Vec<(String, String)> },
    Redirect(RedirectRecord),
    CookieSet(CookieSnapshot),
    CookieDiff(Vec<CookieMutation>),
    Token(TokenClaim),
    FinalPage { url: String, authenticated: bool },
}

#[derive(Clone, Debug, Default)]
pub struct AuthFlowRecorder { events: Vec<AuthEvent> }

impl AuthFlowRecorder {
    pub fn new() -> Self { Self::default() }
    pub fn login_form(&mut self, url: impl Into<String>, fields: Vec<String>) {
        self.events.push(AuthEvent::LoginForm { url: url.into(), fields });
    }
    pub fn request(&mut self, method: impl Into<String>, url: impl Into<String>, headers: Vec<(String, String)>) {
        self.events.push(AuthEvent::Request { method: method.into(), url: url.into(), headers });
    }
    pub fn response(&mut self, url: impl Into<String>, status: u16, headers: Vec<(String, String)>) {
        self.events.push(AuthEvent::Response { url: url.into(), status, headers });
    }
    pub fn redirect(&mut self, from: impl Into<String>, to: impl Into<String>, code: u16, headers: Vec<(String, String)>) {
        self.events.push(AuthEvent::Redirect(RedirectRecord { from_url: from.into(), to_url: to.into(), status_code: code, headers }));
    }
    pub fn cookie_set(&mut self, cookie: CookieSnapshot) { self.events.push(AuthEvent::CookieSet(cookie)); }
    pub fn cookie_diff(&mut self, diff: Vec<CookieMutation>) { self.events.push(AuthEvent::CookieDiff(diff)); }
    pub fn token(&mut self, token: TokenClaim) { self.events.push(AuthEvent::Token(token)); }
    pub fn final_page(&mut self, url: impl Into<String>, authed: bool) {
        self.events.push(AuthEvent::FinalPage { url: url.into(), authenticated: authed });
    }
    pub fn events(&self) -> &[AuthEvent] { &self.events }
    pub fn redirects(&self) -> Vec<RedirectRecord> {
        self.events.iter().filter_map(|e| if let AuthEvent::Redirect(r) = e { Some(r.clone()) } else { None }).collect()
    }
}
