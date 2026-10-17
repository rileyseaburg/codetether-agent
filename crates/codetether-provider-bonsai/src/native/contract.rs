//! Reject unsupported Prism transforms before attempting Candle inference.
use super::Index;
use anyhow::{Result, ensure};
use serde_json::Value;
pub(super) fn validate(index: &Index) -> Result<()> {
    let m = &index.metadata;
    for (key, expected) in [
        ("general.architecture", "qwen35"),
        (
            "prism.hadamard.transform",
            "normalized-sylvester-walsh-hadamard",
        ),
        ("prism.hadamard.axis", "input-last-dimension"),
        ("prism.hadamard.sign_mode", "explicit"),
    ] {
        ensure!(
            m.get(key).and_then(Value::as_str) == Some(expected),
            "Unsupported Bonsai metadata: {key}"
        );
    }
    for (key, expected) in [
        ("prism.hadamard.version", 1),
        ("prism.hadamard.block_size", 1024),
    ] {
        ensure!(
            m.get(key).and_then(Value::as_u64) == Some(expected),
            "Unsupported Bonsai metadata: {key}"
        );
    }
    ensure!(
        m.get("prism.hadamard.gdn_v_grouped")
            .and_then(Value::as_bool)
            == Some(true),
        "Bonsai GDN head layout must be explicit and grouped"
    );
    super::contract_signs::validate(index)?;
    super::contract_weights::validate(index)?;
    super::contract_inverse::validate(index)?;
    Ok(())
}
