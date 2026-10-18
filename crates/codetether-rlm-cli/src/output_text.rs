//! Human-readable RLM command report.
pub(super) fn write(output: &crate::output::Output<'_>) -> anyhow::Result<()> {
    let crate::output::Output {
        args,
        runs,
        result,
        oracle_status,
        ..
    } = output;

    println!("## RLM Analysis Result\n");
    println!("**Query:** {}\n", args.query);
    println!("**Answer:**\n{}\n", result.answer);
    println!("{}", oracle_status);
    println!("---");
    println!(
        "*Iterations: {} | Sub-queries: {} | Time: {}ms*",
        result.iterations,
        result.sub_queries.len(),
        result.stats.elapsed_ms
    );
    if let Some((_, trace_steps, trace_summary)) = runs.first() {
        println!(
            "\n*Trace steps: {} | Context budget used: {:.1}%*",
            trace_steps.len(),
            trace_summary.budget_used_percent
        );
    }

    if !result.sub_queries.is_empty() {
        println!("\n### Sub-queries made:");
        for (i, sq) in result.sub_queries.iter().enumerate() {
            println!("{}. {} -> {} tokens", i + 1, sq.query, sq.tokens_used);
        }
    }
    Ok(())
}
