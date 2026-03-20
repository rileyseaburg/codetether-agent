use super::ExecutionProvenance;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn sign_provenance(provenance: &ExecutionProvenance) -> Option<String> {
    let secret = std::env::var("CODETETHER_SIGNING_SECRET").ok()?;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).ok()?;
    mac.update(payload(provenance).as_bytes());
    Some(hex::encode(mac.finalize().into_bytes()))
}

fn payload(provenance: &ExecutionProvenance) -> String {
    format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        provenance.provenance_id,
        part(provenance.session_id.as_deref()),
        part(provenance.task_id.as_deref()),
        part(provenance.run_id.as_deref()),
        part(provenance.attempt_id.as_deref()),
        part(provenance.identity.tenant_id.as_deref()),
        part(provenance.identity.agent_identity_id.as_deref()),
        provenance.identity.agent_name,
        provenance.identity.origin.as_str(),
        part(provenance.identity.worker_id.as_deref()),
        part(provenance.identity.key_id.as_deref()),
        part(provenance.identity.github_installation_id.as_deref()),
        part(provenance.identity.github_app_id.as_deref()),
    )
}

fn part(value: Option<&str>) -> &str {
    value.unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::sign_provenance;
    use crate::provenance::{ExecutionOrigin, ExecutionProvenance};

    #[test]
    fn signs_provenance_with_secret() {
        unsafe {
            std::env::set_var("CODETETHER_SIGNING_SECRET", "test-secret");
        }
        let provenance = ExecutionProvenance::for_operation("worker", ExecutionOrigin::Worker);
        let signature = sign_provenance(&provenance);
        unsafe {
            std::env::remove_var("CODETETHER_SIGNING_SECRET");
        }
        assert!(signature.is_some());
    }
}
