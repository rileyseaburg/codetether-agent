//! Page-level keyboard actions.

use crate::browser_agent::action::ActionReport;
use crate::browser_agent::interact::focus;
use crate::browser_agent::keyboard::{keyboard_edit, keyboard_script, KeyboardKey};
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::{prepare, retry};
use crate::browser_session::TraceEvent;

impl BrowserPage {
    /// Press one keyboard key against the element matched by `locator`.
    pub fn press<K>(&mut self, locator: &Locator, key: K) -> Result<ActionReport, String>
    where
        K: TryInto<KeyboardKey>,
        K::Error: ToString,
    {
        let key = key.try_into().map_err(|err| err.to_string())?;
        let (resolved, checks) = if key.needs_editable() {
            retry::run(self, "press", locator, |page| prepare::fill(page, locator))?
        } else {
            retry::run(self, "press", locator, |page| prepare::click(page, locator))?
        };
        let replacement = keyboard_edit::replacement(&resolved, &key);
        let allowed = self.eval_js(&keyboard_script::press(
            &resolved.dom.path,
            &key,
            replacement.as_deref(),
        ))?;
        if matches!(key, KeyboardKey::Tab) && allowed.truthy() {
            self.session.focus = Some(focus::selector_for(
                &resolved.dom.path,
                &resolved.dom.element,
            ));
            self.focus_next()?;
        }
        self.session
            .trace
            .push(TraceEvent::new("press", format!("{:?}", locator.kind)));
        Ok(ActionReport::new(
            "press",
            format!("{:?}", locator.kind),
            resolved.bounds,
            checks,
        ))
    }
}
