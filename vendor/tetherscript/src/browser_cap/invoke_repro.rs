//! Minimal repro export for browser traces.

use crate::value::Value;

use super::super::authority::BrowserAuthority;

impl BrowserAuthority {
    pub(crate) fn minimal_repro(&self) -> Value {
        let trace = self.trace.borrow();
        let mut lines = vec!["fn repro(browser) {".to_string()];
        for event in &trace.actions {
            lines.push(format!("    // {}", event.method));
        }
        lines.push("    return browser.trace()?".into());
        lines.push("}".into());
        super::super::value::str_value(lines.join("\n"))
    }
}
