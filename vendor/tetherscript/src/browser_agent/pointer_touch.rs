//! Deterministic touch action support.

use crate::browser_agent::action::{ActionReport, BoundingBox};
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::{prepare, retry};
use crate::browser_session::TraceEvent;

impl BrowserPage {
    /// Dispatch a deterministic touchstart/touchend tap sequence.
    pub fn touch_tap(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        let (resolved, checks) = retry::run(self, "touch_tap", locator, |page| {
            prepare::click(page, locator)
        })?;
        self.eval_js(&touch_script(&resolved.dom.path, resolved.bounds))?;
        self.session
            .trace
            .push(TraceEvent::new("touch_tap", format!("{:?}", locator.kind)));
        Ok(ActionReport::new(
            "touch_tap",
            format!("{:?}", locator.kind),
            resolved.bounds,
            checks,
        ))
    }
}

fn touch_script(path: &[usize], bounds: BoundingBox) -> String {
    format!(
        "let n={};let t={{type:'touchstart',touches:[{{{}}}],\
         targetTouches:[{{{}}}],changedTouches:[{{{}}}]}};\
         n.dispatchEvent(t);n.dispatchEvent({{type:'touchend',touches:[],\
         targetTouches:[],changedTouches:t.changedTouches}});",
        crate::browser_agent::keyboard_escape::node(path),
        super::pointer_event_fields::touch(bounds),
        super::pointer_event_fields::touch(bounds),
        super::pointer_event_fields::touch(bounds)
    )
}
