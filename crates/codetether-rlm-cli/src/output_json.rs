//! Machine-readable RLM command report.
pub(super) fn write(output: &crate::output::Output<'_>) -> anyhow::Result<()> {
    let crate::output::Output {
        args,
        runs,
        provider_name,
        model,
        content_type,
        input_tokens,
        result,
        oracle_json,
        ..
    } = output;
    let run_count = args.consensus_runs;

    let trace_steps_json = runs
        .first()
        .map(|(_, t, _)| serde_json::to_value(t).unwrap_or_default())
        .unwrap_or_else(|| serde_json::json!([]));
    let context_trace_json = runs
        .first()
        .map(|(_, _, c)| serde_json::to_value(c).unwrap_or_default())
        .unwrap_or_else(|| serde_json::json!({}));
    let context_traces: Vec<serde_json::Value> = runs
        .iter()
        .map(|(_, _, c)| serde_json::to_value(c).unwrap_or_default())
        .collect();
    let output = serde_json::json!({
        "query": args.query,
        "provider": provider_name,
        "model": model,
        "content_type": format!("{:?}", content_type),
        "input_tokens": input_tokens,
        "answer": result.answer,
        "iterations": result.iterations,
        "sub_queries": result.sub_queries.len(),
        "stats": result.stats,
        "trace_steps": trace_steps_json,
        "context_trace": context_trace_json,
        "runs": run_count,
        "context_traces": context_traces,
        "oracle": oracle_json,
    });
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
