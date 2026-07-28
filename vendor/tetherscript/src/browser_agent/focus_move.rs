//! Page focus-order APIs.

use super::{order, path, script, FocusTarget};
use crate::browser_agent::page::BrowserPage;
use crate::browser_session::TraceEvent;

impl BrowserPage {
    /// Return focusable elements in deterministic tab order.
    pub fn focus_order(&self) -> Vec<FocusTarget> {
        order::collect(&self.session.document)
    }

    /// Move focus to the next focusable element, wrapping at the end.
    pub fn focus_next(&mut self) -> Result<Option<FocusTarget>, String> {
        self.move_focus(true)
    }

    /// Move focus to the previous focusable element, wrapping at the start.
    pub fn focus_previous(&mut self) -> Result<Option<FocusTarget>, String> {
        self.move_focus(false)
    }

    fn move_focus(&mut self, forward: bool) -> Result<Option<FocusTarget>, String> {
        let targets = self.focus_order();
        if targets.is_empty() {
            self.session.focus = None;
            return Ok(None);
        }
        let current = self.session.focus.clone();
        let target = next_target(&targets, current.as_deref(), forward);
        self.eval_js(&script::focus(&target.path))?;
        self.session.focus = Some(target.selector.clone());
        self.session
            .trace
            .push(TraceEvent::new("focus", &target.selector));
        Ok(Some(target))
    }
}

fn next_target(targets: &[FocusTarget], current: Option<&str>, forward: bool) -> FocusTarget {
    let index = current.and_then(|current| focused_index(targets, current));
    let next = match (index, forward) {
        (Some(index), true) => (index + 1) % targets.len(),
        (Some(0), false) | (None, false) => targets.len() - 1,
        (Some(index), false) => index - 1,
        (None, true) => 0,
    };
    targets[next].clone()
}

fn focused_index(targets: &[FocusTarget], current: &str) -> Option<usize> {
    targets
        .iter()
        .position(|target| current == target.selector || current == path::path_key(&target.path))
}
