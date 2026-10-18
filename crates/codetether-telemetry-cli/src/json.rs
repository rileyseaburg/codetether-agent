//! JSON telemetry report with optional recent and filtered records.
use codetether_cli_options::StatsArgs;
use codetether_telemetry::PersistentStats;
pub(super) fn print(args: &StatsArgs, persistent: &PersistentStats) -> anyhow::Result<()> {
    let mut output = serde_json::json!({
        "tool_executions": persistent.stats,
        "token_usage": {
            "total_input": persistent.stats.total_input_tokens,
            "total_output": persistent.stats.total_output_tokens,
            "requests": persistent.stats.total_requests,
        }
    });

    if args.tools || args.all {
        let recent: Vec<_> = persistent.recent(args.limit).into_iter().collect();
        output["recent_executions"] = serde_json::json!(recent);
    }

    if args.files || args.all {
        let changes = persistent.all_file_changes();
        let recent_changes: Vec<_> = changes.iter().rev().take(args.limit).collect();
        output["recent_file_changes"] = serde_json::json!(recent_changes);
    }

    if let Some(ref tool_name) = args.tool {
        let tool_execs = persistent.by_tool(tool_name);
        output["tool_filter"] = serde_json::json!(tool_execs);
    }

    if let Some(ref file_path) = args.file {
        let file_execs = persistent.by_file(file_path);
        output["file_filter"] = serde_json::json!(file_execs);
    }

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
