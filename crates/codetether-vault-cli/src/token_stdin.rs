//! Bounded, non-echoing pipe input for automation.
use anyhow::{Result, ensure};
use std::io::{IsTerminal, Read};
pub(super) fn read() -> Result<String> {
    ensure!(
        !std::io::stdin().is_terminal(),
        "--stdin expects a pipe, not an echoed terminal"
    );
    let mut value = String::new();
    std::io::stdin().take(16385).read_to_string(&mut value)?;
    ensure!(value.len() <= 16384, "Vault token input is too large");
    Ok(value.trim().to_owned())
}
