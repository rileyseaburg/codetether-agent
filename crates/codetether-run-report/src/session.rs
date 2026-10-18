//! session result output for the selected CLI format.
/// Print the session result in the selected CLI format.
/// # Arguments
/// * `format` - jsonl, json, or another value selecting ordinary text.
/// * `result` - Prompt result, including its JSONL session identity.
/// * `usage` - Usage omitted from JSONL when token counts are zero.
/// * `session_id` - Native session identity for the text continuation hint.
/// # Returns
/// Unit after the requested output completes.
/// # Errors
/// Returns serialization or JSONL output errors.
/// # Examples
/// ```no_run
/// # fn example(result: &codetether_session_events::SessionResult, usage: &codetether_provider_core::Usage) -> anyhow::Result<()> {
/// codetether_run_report::render_session("jsonl", result, usage, "session-1")?;
/// # Ok(()) }
/// ```
pub fn render_session(
    format: &str,
    result: &codetether_session_events::SessionResult,
    usage: &codetether_provider_core::Usage,
    session_id: &str,
) -> anyhow::Result<()> {
    match format {
        "jsonl" => codetether_run_jsonl::write_completed(
            Some(result.session_id.as_str()),
            &result.text,
            codetether_run_jsonl::usage(usage),
        )?,
        "json" => {
            println!("{}", serde_json::to_string_pretty(result)?);
        }
        _ => {
            println!("{}", result.text);
            // Show session ID for continuation
            eprintln!(
                "\n[Session: {} | Continue with: codetether run -c \"...\"]",
                session_id
            );
        }
    }
    Ok(())
}
