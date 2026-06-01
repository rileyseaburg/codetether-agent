use super::ExecutionProvenance;

pub fn push_run_trailers(
    trailers: &mut Vec<(&'static str, String)>,
    provenance: &ExecutionProvenance,
) {
    push_opt(
        trailers,
        "CodeTether-Session-ID",
        provenance.session_id.clone(),
    );
    push_opt(trailers, "CodeTether-Task-ID", provenance.task_id.clone());
    push_opt(trailers, "CodeTether-Run-ID", provenance.run_id.clone());
    push_opt(
        trailers,
        "CodeTether-Attempt-ID",
        provenance.attempt_id.clone(),
    );
    push_opt(
        trailers,
        "CodeTether-Signature",
        super::sign_provenance(provenance),
    );
}

fn push_opt(
    trailers: &mut Vec<(&'static str, String)>,
    label: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        trailers.push((label, value));
    }
}
