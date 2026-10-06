//! Stable DNS-safe names for sub-agent pods.

use sha2::{Digest, Sha256};

pub fn pod_name(subagent_id: &str) -> String {
    let mut sanitized: String = subagent_id
        .to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    sanitized = sanitized.trim_matches('-').to_string();
    if sanitized.is_empty() {
        sanitized = "subagent".into();
    }
    let hash = hex::encode(Sha256::digest(subagent_id.as_bytes()));
    const PREFIX: &str = "codetether-subagent-";
    let max = 63usize.saturating_sub(PREFIX.len() + 1 + 10);
    let mut name: String = sanitized.chars().take(max.max(1)).collect();
    name = name.trim_matches('-').to_string();
    if name.is_empty() {
        name = "subagent".into();
    }
    format!("{PREFIX}{name}-{}", &hash[..10])
}

#[cfg(test)]
mod tests {
    use super::pod_name;

    #[test]
    fn name_is_dns_safe_stable_and_collision_resistant() {
        let first = pod_name("SubTask_ABC/123");
        assert_eq!(first, pod_name("SubTask_ABC/123"));
        assert!(first.len() <= 63);
        assert!(
            first
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        );
        assert_ne!(
            pod_name("subtask-aaaaaaaa-1111"),
            pod_name("subtask-aaaaaaaa-2222")
        );
    }
}
