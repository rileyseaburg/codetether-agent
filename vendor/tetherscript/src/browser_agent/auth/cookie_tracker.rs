//! Cookie mutation tracking across navigation.

use super::session_state::CookieSnapshot;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CookieMutation {
    pub name: String, pub domain: String, pub path: String,
    pub before: Option<CookieSnapshot>, pub after: Option<CookieSnapshot>,
}

#[derive(Clone, Debug, Default)]
pub struct CookieTracker {
    before: Vec<CookieSnapshot>,
    after: Vec<CookieSnapshot>,
}

impl CookieTracker {
    pub fn new() -> Self { Self::default() }
    pub fn capture_before(&mut self, cookies: Vec<CookieSnapshot>) { self.before = cookies; }
    pub fn capture_after(&mut self, cookies: Vec<CookieSnapshot>) { self.after = cookies; }
    pub fn diff(&self) -> Vec<CookieMutation> {
        let mut out = Vec::new();
        for b in &self.before {
            match find(&self.after, b) {
                Some(a) if a != b => out.push(mutation(b, Some(b.clone()), Some(a.clone()))),
                None => out.push(mutation(b, Some(b.clone()), None)),
                _ => {}
            }
        }
        for a in &self.after {
            if find(&self.before, a).is_none() {
                out.push(mutation(a, None, Some(a.clone())));
            }
        }
        out
    }
}

fn find<'a>(v: &'a [CookieSnapshot], k: &CookieSnapshot) -> Option<&'a CookieSnapshot> {
    v.iter().find(|c| c.name == k.name && c.domain == k.domain && c.path == k.path)
}
fn mutation(k: &CookieSnapshot, before: Option<CookieSnapshot>, after: Option<CookieSnapshot>) -> CookieMutation {
    CookieMutation { name: k.name.clone(), domain: k.domain.clone(), path: k.path.clone(), before, after }
}
