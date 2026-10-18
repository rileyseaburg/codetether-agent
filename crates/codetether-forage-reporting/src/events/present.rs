//! Forage present reporting.
use super::*;
use serde::Serialize;

impl RunEvents<'_> {
    /// Print the selected cycle as JSON or quiet-gated human output.
    pub fn present(&self, cycle: usize, selected: &[ForageOpportunity]) -> Result<()> {
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
