//! Deterministic network replay.

use super::capture::{CapturedExchange, CapturedResponse, Headers};

#[derive(Clone, Debug, Default)]
pub struct ReplayOptions { pub headers: Headers, pub body: Option<Vec<u8>>, pub delay_ms: u64 }

#[derive(Clone, Debug, PartialEq)]
pub struct ReplayedExchange { pub request: CapturedExchange, pub response: Option<CapturedResponse>, pub delay_ms: u64 }

pub struct ReplayPlan { exchanges: Vec<CapturedExchange>, cursor: usize, default_delay_ms: u64 }

impl ReplayPlan {
    pub fn new(exchanges: Vec<CapturedExchange>) -> Self { Self { exchanges, cursor: 0, default_delay_ms: 0 } }
    pub fn with_default_delay(mut self, ms: u64) -> Self { self.default_delay_ms = ms; self }
    pub fn reset(&mut self) { self.cursor = 0; }
    pub fn next(&mut self, opts: ReplayOptions) -> Option<ReplayedExchange> {
        let mut ex = self.exchanges.get(self.cursor)?.clone(); self.cursor += 1;
        for (k, v) in opts.headers { upsert(&mut ex.request.headers, k, v); }
        if let Some(b) = opts.body { ex.request.body = b; }
        let delay = if opts.delay_ms == 0 { self.default_delay_ms } else { opts.delay_ms };
        Some(ReplayedExchange { response: ex.response.clone(), request: ex, delay_ms: delay })
    }
    pub fn replay_match(&self, method: &str, url: &str, opts: ReplayOptions) -> Option<ReplayedExchange> {
        let mut plan = ReplayPlan::new(self.exchanges.iter()
            .filter(|e| e.request.method == method && e.request.url == url).cloned().collect());
        plan.default_delay_ms = self.default_delay_ms; plan.next(opts)
    }
}

fn upsert(h: &mut Headers, k: String, v: String) {
    if let Some((_, old)) = h.iter_mut().find(|(n, _)| n.eq_ignore_ascii_case(&k)) { *old = v } else { h.push((k, v)) }
}
