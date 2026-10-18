//! Bound cycle execution, sleep and normal shutdown reporting.
use super::*;
use std::time::Duration;

impl<H: Host> RunLoop<'_, H> {
    pub(crate) async fn run(mut self) -> Result<ForageRunSummary> {
        let top = self.args.top.clamp(1, 50);
        let interval_secs = self.args.interval_secs.clamp(5, 86_400);
        let mut cycle: usize = 0;
        let mut summary = summary::initial(self.args);

        loop {
            // Only check S3 health if S3 is required
            if self.require_s3 {
                ensure_s3_sync_alive(&mut *self.s3_sync_handle).await?;
            }
            cycle = cycle.saturating_add(1);
            let cycle_task_id = self.cycle(cycle, top, &mut summary).await?;

            let keep_running =
                self.args.loop_mode && (self.args.max_cycles == 0 || cycle < self.args.max_cycles);
            self.events.complete(&cycle_task_id, keep_running);

            if !keep_running {
                break;
            }

            self.events.sleeping(cycle, interval_secs, &cycle_task_id);
            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
        }
        self.events.stop(cycle).await;

        Ok(summary)
    }
}
