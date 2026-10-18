//! Perform discovery, reporting and selected-item execution for one cycle.
use super::*;
impl<H: Host> RunLoop<'_, H> {
    pub(super) async fn cycle(
        &mut self,
        cycle: usize,
        top: usize,
        summary: &mut ForageRunSummary,
    ) -> Result<String> {
        let cycle_task_id = format!("forage-cycle-{cycle}");
        self.events.scan(&cycle_task_id);
        let selected =
            discover::discover::<H>(self.repo, self.moonshot_rubric, self.args.execute, top)
                .await?;
        self.events.selected(&cycle_task_id, cycle, top, &selected);

        self.events.present(cycle, &selected)?;
        summary.cycles_completed = cycle;
        summary.total_selected = summary.total_selected.saturating_add(selected.len());
        summary.last_cycle_selected = ForageRunSummary::summarize_selection(&selected);
        self.events.cycle(cycle, top, &selected).await;
        flush_bus_observer(&mut *self.observer, cycle, self.args.json);

        execute_selected::execute_selected::<H>(
            self.args,
            self.repo,
            self.events,
            &selected,
            summary,
            cycle,
        )
        .await?;

        Ok(cycle_task_id)
    }
}
