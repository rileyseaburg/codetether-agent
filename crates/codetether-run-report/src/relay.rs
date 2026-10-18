//! relay result output for the selected CLI format.
/// Print the relay result in the selected CLI format.
/// # Arguments
/// * `format` - jsonl, json, or another value selecting ordinary text.
/// * `relay_result` - Final relay status, summary and evidence counters.
/// # Returns
/// Unit after the requested output completes.
/// # Errors
/// Returns serialization or JSONL output errors.
/// # Examples
/// ```no_run
/// # fn example(result: &codetether_relay_report::AutochatCliResult) -> anyhow::Result<()> {
/// codetether_run_report::render_relay("json", result)?;
/// # Ok(()) }
/// ```
pub fn render_relay(
    format: &str,
    relay_result: &codetether_relay_report::AutochatCliResult,
) -> anyhow::Result<()> {
    match format {
        "jsonl" => {
            if let Some(failure) = &relay_result.failure {
                codetether_run_jsonl::write_failed_response(failure, Some(&relay_result.summary))?;
            } else {
                codetether_run_jsonl::write_completed(None, &relay_result.summary, None)?;
            }
        }
        "json" => println!("{}", serde_json::to_string_pretty(relay_result)?),
        _ => {
            println!("{}", relay_result.summary);
            if let Some(failure) = &relay_result.failure {
                eprintln!("\nFailure detail: {}", failure);
            }
            eprintln!(
                "\n[Relay: {} | Model: {}]",
                relay_result.relay_id, relay_result.model
            );
        }
    }
    Ok(())
}
