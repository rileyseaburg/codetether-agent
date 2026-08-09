//! Script-execution reporting for native navigation.
//!
//! The native backend interprets page JavaScript rather than running a real
//! engine, so some scripts fail to execute. Silently discarding that failure
//! makes a navigation look successful while no handlers are attached, which
//! leads callers to blame the page instead of the backend. Navigation still
//! succeeds — the page loaded — but the script outcome is reported.

use serde_json::{Value, json};
use tetherscript::browser_agent::BrowserPage;

/// Runs page scripts and returns a report of what actually executed.
pub(super) fn run_and_report(page: &mut BrowserPage) -> Value {
    match page.run_scripts() {
        Ok(()) => json!({"ok": true, "scripts_executed": true}),
        Err(message) => {
            tracing::warn!(
                error = %message,
                "Native browser could not execute page scripts; \
                 event handlers and injected scripts are not active"
            );
            json!({
                "ok": true,
                "scripts_executed": false,
                "backend": "tetherscript-native",
                "script_error": message,
                "hint": "Native evaluation does not run all page JavaScript. \
                         Use a real browser for runtime behavior proof.",
            })
        }
    }
}
