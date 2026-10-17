//! Crash reporting maybe prompt for consent.
use super::*;
/// Collect optional crash-report consent when interactive prompting is allowed.
///
/// # Arguments
/// * `config` - Current application configuration.
/// * `allow_prompt` - Whether this caller allows terminal interaction.
/// # Returns
/// Reloaded configuration after a saved choice, or the existing configuration.
/// # Examples
/// ```no_run
/// # async fn example(config: &codetether_config::Config) {
/// let updated = codetether_crash::maybe_prompt_for_consent(config, false).await;
/// let _ = updated;
/// # }
/// ```
pub async fn maybe_prompt_for_consent(config: &Config, allow_prompt: bool) -> Config {
    if !allow_prompt {
        return config.clone();
    }

    if config.telemetry.crash_reporting.is_some() || config.telemetry.crash_reporting_prompted() {
        return config.clone();
    }

    println!();
    println!("CodeTether Optional Crash Reporting");
    println!("- Helps us fix catastrophic crashes faster.");
    println!("- Sends panic message, stack trace, app version, OS/arch, and command.");
    println!("- Does not intentionally include API keys or source files.");
    println!(
        "- You can change this any time with `codetether config --set telemetry.crash_reporting=true|false`."
    );

    let enabled = match prompt_yes_no("Enable crash reporting now? [y/N]: ") {
        Ok(v) => v,
        Err(err) => {
            tracing::warn!(error = %err, "Failed to collect crash reporting consent input");
            return config.clone();
        }
    };

    if let Err(err) = persist_consent_choice(enabled).await {
        tracing::warn!(error = %err, "Failed to persist crash reporting consent choice");
        return config.clone();
    }

    if enabled {
        println!("Crash reporting enabled.");
        if env_non_empty(ENV_CRASH_REPORT_AUTH_TOKEN).is_none()
            && env_non_empty(ENV_CRASH_REPORT_API_KEY).is_none()
        {
            println!(
                "If your telemetry endpoint requires auth, set {} or {}.",
                ENV_CRASH_REPORT_AUTH_TOKEN, ENV_CRASH_REPORT_API_KEY
            );
        }
    } else {
        println!("Crash reporting disabled.");
    }

    match Config::load().await {
        Ok(updated) => updated,
        Err(err) => {
            tracing::warn!(error = %err, "Failed to reload config after consent update");
            config.clone()
        }
    }
}
