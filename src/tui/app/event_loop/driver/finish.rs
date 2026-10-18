//! Consume loop resources through the existing shutdown helper.
use super::Native;
impl codetether_tui_reactor::driver::Finish for Native<'_> {
    async fn finish(self) {
        super::super::shutdown::finish(self.worker_bridge, &self.runtime, self.setup.mux_status)
            .await;
    }
}
