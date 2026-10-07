use super::ExecutionProvenance;

pub fn push_identity_trailers(
    trailers: &mut Vec<(&'static str, String)>,
    provenance: &ExecutionProvenance,
) {
    push_opt(
        trailers,
        "CodeTether-Agent-Identity",
        provenance.identity.agent_identity_id.clone(),
    );
    push_opt(
        trailers,
        "CodeTether-Tenant-ID",
        provenance.identity.tenant_id.clone(),
    );
    push_opt(
        trailers,
        "CodeTether-Worker-ID",
        provenance.identity.worker_id.clone(),
    );
    push_opt(
        trailers,
        "CodeTether-Key-ID",
        provenance.identity.key_id.clone(),
    );
    push_opt(
        trailers,
        "CodeTether-GitHub-Installation-ID",
        provenance.identity.github_installation_id.clone(),
    );
    push_opt(
        trailers,
        "CodeTether-GitHub-App-ID",
        provenance.identity.github_app_id.clone(),
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
