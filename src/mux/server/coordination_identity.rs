//! Bounds checking for client-supplied lease owner and agent identities.

pub(super) fn valid(value: &str) -> anyhow::Result<()> {
    anyhow::ensure!(
        !value.trim().is_empty(),
        "coordination identity cannot be empty"
    );
    anyhow::ensure!(value.len() <= 256, "coordination identity is too long");
    Ok(())
}
