use crate::provenance::{ExecutionOrigin, ExecutionProvenance};

pub(super) fn worker_provenance(
    task_id: &str,
    provenance: Option<&ExecutionProvenance>,
) -> ExecutionProvenance {
    provenance
        .cloned()
        .unwrap_or_else(|| default_worker_provenance(task_id))
}

fn default_worker_provenance(task_id: &str) -> ExecutionProvenance {
    let mut provenance = ExecutionProvenance::for_operation("worker", ExecutionOrigin::Worker);
    provenance.task_id = Some(task_id.to_string());
    provenance
}
