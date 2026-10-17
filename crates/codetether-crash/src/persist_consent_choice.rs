//! Crash reporting persist consent choice.
use super::*;
pub(super) async fn persist_consent_choice(enabled: bool) -> Result<()> {
    let enabled_value = if enabled { "true" } else { "false" };
    Config::set("telemetry.crash_reporting", enabled_value).await?;
    Config::set("telemetry.crash_reporting_prompted", "true").await?;
    Ok(())
}
