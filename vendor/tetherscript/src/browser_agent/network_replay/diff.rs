//! Diff two network captures to find changes.

use super::capture::CapturedExchange;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkDiff { pub added: Vec<String>, pub removed: Vec<String>, pub changed: Vec<ExchangeDiff> }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExchangeDiff { pub key: String, pub fields: Vec<String> }

/// Diff two capture sets, reporting added, removed, and changed exchanges.
pub fn diff_captures(a: &[CapturedExchange], b: &[CapturedExchange]) -> NetworkDiff {
    let (mut added, mut removed, mut changed) = (vec![], vec![], vec![]);
    for old in a { if !b.iter().any(|n| key(n) == key(old)) { removed.push(key(old)); } }
    for new in b {
        if let Some(old) = a.iter().find(|o| key(o) == key(new)) {
            let f = fields(old, new); if !f.is_empty() { changed.push(ExchangeDiff { key: key(new), fields: f }); }
        } else { added.push(key(new)); }
    }
    NetworkDiff { added, removed, changed }
}

fn fields(a: &CapturedExchange, b: &CapturedExchange) -> Vec<String> {
    let mut f = vec![];
    if a.request.headers != b.request.headers { f.push("request.headers".into()); }
    if a.request.body != b.request.body { f.push("request.body".into()); }
    if a.response.as_ref().map(|r| r.status) != b.response.as_ref().map(|r| r.status) { f.push("response.status".into()); }
    if a.response.as_ref().map(|r| &r.headers) != b.response.as_ref().map(|r| &r.headers) { f.push("response.headers".into()); }
    if a.response.as_ref().map(|r| &r.body) != b.response.as_ref().map(|r| &r.body) { f.push("response.body".into()); }
    f
}

pub fn key(e: &CapturedExchange) -> String { format!("{} {}", e.request.method, e.request.url) }
