use super::*;
impl CollapseController {
    pub(super) fn observe(&mut self, observations: &[BranchObservation]) -> ObservationSnapshot {
        let mut changed_files: HashMap<String, HashSet<String>> = HashMap::new();
        let mut changed_lines: HashMap<String, u32> = HashMap::new();
        let mut compile_ok: HashMap<String, bool> = HashMap::new();

        for observation in observations {
            self.first_seen_at
                .entry(observation.subtask_id.clone())
                .or_insert_with(Instant::now);
            changed_files.insert(
                observation.subtask_id.clone(),
                observation.changed_files.clone(),
            );
            changed_lines.insert(observation.subtask_id.clone(), observation.changed_lines);
            compile_ok.insert(observation.subtask_id.clone(), observation.compile_ok);

            self.observe_health(observation);
        }

        ObservationSnapshot {
            changed_files,
            changed_lines,
            compile_ok,
        }
    }
}
