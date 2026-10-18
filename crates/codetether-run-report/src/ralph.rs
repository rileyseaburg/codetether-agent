//! ralph result output for the selected CLI format.
/// Print the Ralph result in the selected CLI format.
/// # Arguments
/// * `format` - jsonl, json, or another value selecting ordinary text.
/// * `ralph_result` - Final PRD execution result.
/// * `task` - Original task for the text formatter.
/// # Returns
/// Unit after the requested output completes.
/// # Errors
/// Returns serialization or JSONL output errors.
/// # Examples
/// ```no_run
/// # fn example(result: &codetether_go_ralph::GoRalphResult) -> anyhow::Result<()> {
/// codetether_run_report::render_ralph("json", result, "task")?;
/// # Ok(()) }
/// ```
pub fn render_ralph(
    format: &str,
    ralph_result: &codetether_go_ralph::GoRalphResult,
    task: &str,
) -> anyhow::Result<()> {
    match format {
        "jsonl" => {
            let response = codetether_go_ralph::format_go_ralph_result(ralph_result, task);
            codetether_run_jsonl::write_completed(None, &response, None)?;
        }
        "json" => println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "passed": ralph_result.passed,
                "total": ralph_result.total,
                "all_passed": ralph_result.all_passed,
                "iterations": ralph_result.iterations,
                "feature_branch": ralph_result.feature_branch,
                "prd_path": ralph_result.prd_path.display().to_string(),
                "status": format!("{:?}", ralph_result.status),
                "stories": ralph_result.stories.iter().map(|s| serde_json::json!({
                    "id": s.id,
                    "title": s.title,
                    "passed": s.passed,
                })).collect::<Vec<_>>(),
            }))?
        ),
        _ => {
            println!(
                "{}",
                codetether_go_ralph::format_go_ralph_result(ralph_result, task)
            );
        }
    }
    Ok(())
}
