//! Deterministic wheel action support.

use crate::browser_agent::action::ActionReport;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::{prepare, retry};
use crate::browser_session::TraceEvent;
use crate::js::JsValue;

impl BrowserPage {
    /// Dispatch a wheel event and apply default viewport scrolling.
    pub fn wheel(
        &mut self,
        locator: &Locator,
        delta_x: i64,
        delta_y: i64,
    ) -> Result<ActionReport, String> {
        let base_scroll = self.session.scroll.clone();
        let (resolved, checks) =
            retry::run(self, "wheel", locator, |page| prepare::click(page, locator))?;
        let script = wheel_script(&resolved.dom.path, delta_x, delta_y, resolved.bounds);
        if self.eval_js(&script)? != JsValue::Bool(false) {
            self.session.scroll.x = axis(base_scroll.x, delta_x);
            self.session.scroll.y = axis(base_scroll.y, delta_y);
        } else {
            self.session.scroll = base_scroll;
        }
        self.session
            .trace
            .push(TraceEvent::new("wheel", format!("{:?}", locator.kind)));
        Ok(ActionReport::new(
            "wheel",
            format!("{:?}", locator.kind),
            resolved.bounds,
            checks,
        ))
    }
}

fn wheel_script(
    path: &[usize],
    delta_x: i64,
    delta_y: i64,
    bounds: crate::browser_agent::action::BoundingBox,
) -> String {
    format!(
        "let n={};let ok=n.dispatchEvent({{type:'wheel',{}}});ok;",
        crate::browser_agent::keyboard_escape::node(path),
        super::pointer_event_fields::wheel(delta_x, delta_y, bounds)
    )
}

fn axis(current: i64, delta: i64) -> i64 {
    current.saturating_add(delta).max(0)
}
