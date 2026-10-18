//! Native shortcut bindings.
mod input;
mod routing;
mod state;
mod view;
use super::{App, Path, TuiSessionHandle};
use codetether_tui_navigation::modified::{Action, Effects};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub runtime: &'a TuiSessionHandle,
}
impl Effects for Native<'_> {
    fn effect(&mut self, action: Action) {
        if input::apply(self.app, &action) || view::apply(self.app, &action) {
            return;
        }
        unreachable!("unmapped shortcut action");
    }
}
