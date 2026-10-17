use super::forgejo_agent_identity;

#[test]
fn matches_the_cross_runtime_contract_vector() {
    assert_eq!(
        forgejo_agent_identity("forge.example", "alice", "default").as_deref(),
        Some("ctforgejo_9e1fbe45a595b21bd9146db4a011cae0de38f193")
    );
}

#[test]
fn canonicalizes_host_and_login() {
    assert_eq!(
        forgejo_agent_identity("HTTPS://Forge.Example/api/v1", "Alice", "default"),
        forgejo_agent_identity("forge.example", "alice", "default")
    );
}

#[test]
fn separates_agent_slots() {
    let author = forgejo_agent_identity("forge.example", "alice", "author");
    let reviewer = forgejo_agent_identity("forge.example", "alice", "reviewer");
    assert_ne!(author, reviewer);
}

#[test]
fn rejects_unsafe_principals() {
    assert!(forgejo_agent_identity("forge.example", "alice/other", "default").is_none());
}

#[test]
fn rejects_leading_punctuation_like_the_server_contract() {
    assert!(forgejo_agent_identity("forge.example", ".alice", "default").is_none());
    assert!(forgejo_agent_identity("forge.example", "alice", "-slot").is_none());
}
