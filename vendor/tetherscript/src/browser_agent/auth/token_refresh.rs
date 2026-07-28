//! Token refresh cycle detection.

use super::session_state::TokenClaim;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TokenRefresh {
    pub kind: String, pub subject: Option<String>,
    pub old_expires_at: Option<i64>, pub new_expires_at: Option<i64>,
}

#[derive(Clone, Debug, Default)]
pub struct TokenRefreshTracker {
    previous: Vec<TokenClaim>,
    refreshes: Vec<TokenRefresh>,
}

impl TokenRefreshTracker {
    pub fn new() -> Self { Self::default() }
    pub fn observe(&mut self, tokens: Vec<TokenClaim>) -> Vec<TokenRefresh> {
        let mut found = Vec::new();
        for new in &tokens {
            if let Some(old) = self.previous.iter().find(|o| same_identity(o, new)) {
                if old.raw_payload != new.raw_payload && newer(old.expires_at, new.expires_at) {
                    found.push(TokenRefresh {
                        kind: new.kind.clone(), subject: new.subject.clone(),
                        old_expires_at: old.expires_at, new_expires_at: new.expires_at,
                    });
                }
            }
        }
        self.previous = tokens;
        self.refreshes.extend(found.clone());
        found
    }
    pub fn refreshes(&self) -> &[TokenRefresh] { &self.refreshes }
}

pub fn expires_within(token: &TokenClaim, now_unix: i64, window_secs: i64) -> bool {
    token.expires_at.map_or(false, |exp| exp > now_unix && exp <= now_unix + window_secs)
}

fn same_identity(a: &TokenClaim, b: &TokenClaim) -> bool {
    a.kind == b.kind && a.issuer == b.issuer && a.subject == b.subject
}
fn newer(old: Option<i64>, new: Option<i64>) -> bool {
    match (old, new) { (Some(o), Some(n)) => n > o, (None, Some(_)) => true, _ => false }
}
