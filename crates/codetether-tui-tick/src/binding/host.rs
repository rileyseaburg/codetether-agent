//! Frontend subsystem drains for one periodic tick.
/// Implementation detail of [crate::declare_frontend_tick].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_tick_host {
    ($app:path; $question:path, $snapshot:path, $autochat:path, $voice:path,
     $swarm:path, $forage:path, $shell:path, $history:path, $symbols:path, $approvals:path) => {
        mod host {
            use $app as App;
            use $crate::{Host, Snapshot};
            pub(super) struct Native<'a>(pub &'a mut App);
            impl Host for Native<'_> {
                fn question_wait(&mut self) {
                    $question(self.0);
                }
                fn snapshot(&self) -> Snapshot {
                    $snapshot(self.0)
                }
                fn autochat(&mut self) {
                    $autochat(&mut self.0.state);
                }
                fn voice(&mut self) {
                    $voice(&mut self.0.state);
                }
                fn ralph(&mut self) -> bool {
                    self.0.state.ralph.drain_events()
                }
                fn swarm(&mut self) -> bool {
                    $swarm(&mut self.0.state.swarm)
                }
                fn forage(&mut self) -> bool {
                    $forage(self.0)
                }
                fn shell(&mut self) -> bool {
                    $shell(self.0)
                }
                fn history(&mut self) -> bool {
                    $history(self.0)
                }
                fn symbols(&mut self) -> bool {
                    $symbols(self.0)
                }
                fn approvals(&mut self) -> bool {
                    $approvals(self.0)
                }
                fn mark_redraw(&mut self, changed: bool) {
                    self.0.state.needs_redraw |= changed;
                }
            }
        }
    };
}
