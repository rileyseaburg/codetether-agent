//! Opt-in synthetic role diagnostic using the application's normal Vault login.
//! Credentials are piped in memory, never printed or persisted.
use codetether_secrets::SecretsManager;
use std::{
    io::Write,
    process::{Command, Stdio},
};

#[tokio::test]
#[ignore = "requires configured Vault login and performs six Groq requests"]
async fn groq_roles_live() {
    let manager = SecretsManager::from_env()
        .await
        .expect("Vault initialization");
    let secrets = manager
        .get_provider_secrets("groq")
        .await
        .expect("Groq credential lookup")
        .expect("Groq configured");
    let key = secrets.api_key.expect("Groq API key configured");
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut child = Command::new("node")
        .arg(root.join("artifacts/groq-qwen-roles/probe.mjs"))
        .stdin(Stdio::piped())
        .spawn()
        .expect("start synthetic probe");
    child
        .stdin
        .take()
        .expect("private pipe")
        .write_all(
            serde_json::json!({"data":{"data":{"api_key":key}}})
                .to_string()
                .as_bytes(),
        )
        .expect("write private pipe");
    assert!(child.wait().expect("probe exit").success());
}
