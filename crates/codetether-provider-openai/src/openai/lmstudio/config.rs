//! Validated LM Studio policy; unrelated providers never read these variables.
use anyhow::{Context, Result, ensure};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Config {
    pub(super) output: usize,
    pub(super) waiting: usize,
    pub(super) wait: Duration,
    pub(super) generation: Duration,
}
impl Config {
    pub(super) fn load() -> Result<Self> {
        Ok(Self {
            output: number("CODETETHER_LMSTUDIO_MAX_OUTPUT_TOKENS", 4096, 1, 32768)?,
            waiting: number("CODETETHER_LMSTUDIO_QUEUE_CAPACITY", 16, 0, 128)?,
            wait: Duration::from_secs(
                number("CODETETHER_LMSTUDIO_QUEUE_TIMEOUT_SECS", 120, 1, 600)? as u64,
            ),
            generation: Duration::from_secs(number(
                "CODETETHER_LMSTUDIO_REQUEST_TIMEOUT_SECS",
                600,
                1,
                600,
            )? as u64),
        })
    }
}
fn number(name: &str, default: usize, min: usize, max: usize) -> Result<usize> {
    match std::env::var(name) {
        Ok(value) => parse(name, &value, min, max),
        Err(std::env::VarError::NotPresent) => Ok(default),
        Err(error) => Err(error).with_context(|| format!("Invalid {name}")),
    }
}
pub(super) fn parse(name: &str, text: &str, min: usize, max: usize) -> Result<usize> {
    let value = text
        .parse::<usize>()
        .with_context(|| format!("{name} must be an integer"))?;
    ensure!(
        (min..=max).contains(&value),
        "{name} must be between {min} and {max}"
    );
    Ok(value)
}
#[cfg(test)]
mod tests;
