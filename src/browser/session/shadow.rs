//! Shadow-DOM-aware element lookup for browser session JS.
//!
//! Built-in `document.querySelector` does not pierce shadow roots, so any
//! page that uses web components — Google Colab, Material Web, Lit-based
//! apps, modern Chromium UIs — is invisible to the standard DOM helpers.
//! This module ships a single in-page helper, [`__codetether_dq`], plus a
//! small Rust-side API that wraps any existing JS expression with the
//! installer prelude.
//!
//! # Selector grammar
//!
//! The helper accepts the same CSS selector syntax as `querySelector`,
//! plus the `>>>` shadow-piercing combinator (Playwright / WebDriver BiDi
//! convention). The combinator splits the selector at shadow boundaries:
//!
//! ```text
//! colab-composer-rich-text-field >>> .inputarea
//! ```
//!
//! is interpreted as "find `colab-composer-rich-text-field` in the
//! containing root, descend into its `shadowRoot`, then find `.inputarea`
//! there." This makes shadow boundaries explicit when the caller knows
//! them; for unannotated selectors the helper falls back to a BFS walk
//! through every reachable shadow root.

/// JS source for the in-page shadow-piercing query helper.
///
/// Idempotent: re-evaluating the snippet does not redefine the helper,
/// because the body checks `window.__codetether_dq` first. Returns the
/// helper function itself, so callers can chain `(<install>)(selector)`
/// in a single round-trip.
pub(super) const DEEP_QUERY_INSTALL: &str = r#"
(() => {
    if (window.__codetether_dq) return window.__codetether_dq;
    const SHADOW_SEP = '>>>';
    function dqOnce(root, selector) {
        if (!root || !selector) return null;
        try {
            const direct = (root.querySelector ? root.querySelector(selector) : null);
            if (direct) return direct;
        } catch (_) {
            return null;
        }
        const all = root.querySelectorAll ? root.querySelectorAll('*') : [];
        for (let i = 0; i < all.length; i++) {
            const sr = all[i].shadowRoot;
            if (!sr) continue;
            const found = dqOnce(sr, selector);
            if (found) return found;
        }
        return null;
    }
    function dq(selector) {
        if (typeof selector !== 'string' || !selector) return null;
        if (selector.indexOf(SHADOW_SEP) === -1) {
            return dqOnce(document, selector);
        }
        const segments = selector.split(SHADOW_SEP).map(s => s.trim()).filter(Boolean);
        let root = document;
        for (let i = 0; i < segments.length; i++) {
            const el = dqOnce(root, segments[i]);
            if (!el) return null;
            if (i === segments.length - 1) return el;
            root = el.shadowRoot || el;
        }
        return null;
    }
    window.__codetether_dq = dq;
    return dq;
})()
"#;

/// JS expression that evaluates to the element matched by `selector_lit`,
/// or `null` if no match. `selector_lit` must be a JSON-encoded string
/// literal (i.e. the output of `serde_json::to_string(selector)`).
///
/// Because the helper is installed lazily inside the same expression, the
/// first call after a navigation pays a one-time install cost; subsequent
/// calls reuse `window.__codetether_dq`. Keep this in mind if you need a
/// short-lived expression to avoid leaking a global on a page where the
/// caller has explicitly opted out of injection — currently nothing does.
pub(super) fn dq_call(selector_lit: &str) -> String {
    format!("({DEEP_QUERY_INSTALL})({selector_lit})")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_snippet_is_idempotent_guard() {
        // The first thing the snippet does must be a check on
        // `window.__codetether_dq` so a second eval is a no-op. Without
        // this guard, every call re-installs the helper and observers
        // attached to the previous instance go stale.
        assert!(
            DEEP_QUERY_INSTALL
                .contains("if (window.__codetether_dq) return window.__codetether_dq")
        );
    }

    #[test]
    fn install_snippet_supports_shadow_separator() {
        // The `>>>` combinator is the contract with callers; if the
        // string drifts the API silently regresses to flat-document
        // queries.
        assert!(DEEP_QUERY_INSTALL.contains("'>>>'"));
        assert!(DEEP_QUERY_INSTALL.contains("split(SHADOW_SEP)"));
    }

    #[test]
    fn dq_call_inlines_the_selector_literal() {
        let lit = serde_json::to_string("input.foo").unwrap();
        let js = dq_call(&lit);
        assert!(js.contains("\"input.foo\""));
        // Must be a single self-invoking expression so callers can drop
        // it directly into `page.evaluate` without further wrapping.
        assert!(js.starts_with("("));
        assert!(js.ends_with(")"));
    }
}
