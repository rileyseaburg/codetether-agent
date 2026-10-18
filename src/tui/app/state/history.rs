//! Native entry points for shared composer history transitions.
#[path = "history_push.rs"]
mod history_push;
impl super::AppState {
    pub fn history_prev(&mut self) -> bool {
        self.composer().history_prev()
    }
    pub fn history_next(&mut self) -> bool {
        self.composer().history_next()
    }
}
