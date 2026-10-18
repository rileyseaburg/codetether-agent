//! Native bindings for shared interactive slash dispatch.
use super::*;
use codetether_slash_command::{
    pipeline::{Host, SimpleCommand, TailCommand},
    routing::LeadingCommand,
};
mod leading;
mod simple;
mod tail;
pub(super) struct Dispatch<'a> {
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub session: &'a mut Session,
    pub registry: Option<&'a Arc<ProviderRegistry>>,
}
impl Host for Dispatch<'_> {
    async fn leading(&mut self, kind: LeadingCommand, rest: &str) {
        leading::run(self, kind, rest).await;
    }
    async fn ralph(&mut self, rest: &str) -> bool {
        handle_ralph_subcommand(self.app, self.cwd, self.session, self.registry, rest).await
    }
    fn swarm(&mut self, rest: &str) -> bool {
        crate::tui::swarm_run::handle_swarm_command(self.app, self.session, rest)
    }
    async fn session(&mut self, normalized: &str) -> bool {
        codetether_slash_command::pipeline::session::dispatch(self, normalized).await
    }
    async fn simple(&mut self, kind: SimpleCommand) {
        simple::run(self, kind).await;
    }
    async fn tail(&mut self, kind: TailCommand, rest: &str) {
        tail::run(self, kind, rest).await;
    }
    fn unknown(&mut self, normalized: &str) {
        self.app.state.status = format!("Unknown command: {normalized}");
    }
}
