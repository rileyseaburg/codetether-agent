//! Native actions for the shared session-command dispatcher.
use super::dispatch_host::Dispatch;
impl codetether_slash_command::pipeline::session::Host for Dispatch<'_> {
    fn forage(&mut self, rest: &str) {
        crate::tui::forage_run::handle_forage_command(self.app, self.session, rest);
    }
    async fn goal(&mut self, rest: &str) {
        super::goal::handle(self.app, &self.session.id, rest).await;
    }
    fn a2a(&mut self, rest: &str) {
        super::a2a::handle(self.app, rest);
    }
    async fn undo(&mut self, rest: &str) {
        super::handle_undo_command(self.app, self.session, rest).await;
    }
    async fn detach(&mut self, rest: &str) {
        crate::tui::app::detach::handle_detach_command(self.app, self.session, rest).await;
    }
    async fn fork(&mut self, rest: &str) {
        super::handle_fork_command(self.app, self.cwd, self.session, rest).await;
    }
}
