//! Opt-in live Rust inference plus always-on fixture/parser round-trip tests.
mod credentials;
mod fixture;
mod offline;
mod run;
mod stream;
use crate::GeminiWebProvider;
use codetether_provider_core::Provider;
use serde_json::json;

#[tokio::test]
#[ignore = "live Google inference and configured Vault cookies; requires CODETETHER_GEMINI_PROOF_DIR"]
async fn live_native_fast_tool_roundtrip() -> anyhow::Result<()> {
    let directory = std::env::var_os("CODETETHER_GEMINI_PROOF_DIR")
        .map(std::path::PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("proof directory required"))?;
    std::fs::create_dir_all(&directory)?;
    let provider = GeminiWebProvider::new(credentials::load()?)?;
    let mut turns = Vec::new();
    let outcome = run::exercise(&provider, &mut turns).await;
    let record = match &outcome {
        Ok(proof) => json!({"success":true,"proof":proof}),
        Err(error) => {
            json!({"success":false,"turns":turns,"error":credentials::redact(error, &provider).await})
        }
    };
    let evidence = json!({
        "evidence_level":"native Rust requests to live Google; mocked local tools",
        "requested_model":"gemini-web-fast",
        "resolved_model":provider.resolved_model_identity("gemini-web-fast"),
        "requested_mode":codetether_gemini_web_protocol::GeminiWebProtocol::mode_id("gemini-web-fast"),
        "backend_identity":"not attested",
        "transport":"native reqwest; no curl subprocess",
        "result":record,
    });
    let path = directory.join("native-fast-tools.json");
    std::fs::write(&path, serde_json::to_vec_pretty(&evidence)?)?;
    println!("Native Rust evidence: {}", path.display());
    anyhow::ensure!(
        outcome.is_ok(),
        "native Rust round trip failed; inspect sanitized evidence"
    );
    Ok(())
}
