//! Forage present reporting.
use super::*;
impl RunEvents<'_> {
    pub(in crate::forage) fn present(
        &self,
        cycle: usize,
        selected: &[ForageOpportunity],
    ) -> Result<()> {
        if self.args.json {
            #[derive(Serialize)]
            struct CycleOutput {
                cycle: usize,
                selected: Vec<ForageOpportunity>,
            }
            let payload = CycleOutput {
                cycle,
                selected: selected.to_vec(),
            };
            println!("{}", serde_json::to_string_pretty(&payload)?);
        } else {
            console::print_cycle(cycle, selected, !self.moonshot_rubric.goals.is_empty());
        }

        Ok(())
    }
}
