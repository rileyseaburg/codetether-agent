//! Repeated RLM analysis and per-run context/trace capture.
use crate::RunRecord;
use codetether_cli_options::RlmArgs;
use codetether_rlm as rlm;
pub(super) async fn run(
    args: &RlmArgs,
    content: &str,
    provider: &std::sync::Arc<dyn codetether_provider_core::Provider>,
    model: &str,
) -> anyhow::Result<Vec<RunRecord>> {
    let run_count = args.consensus_runs;
    let run_temperature =
        args.analysis_temperature
            .unwrap_or(if run_count > 1 { 0.75 } else { 0.3 });
    let mut runs: Vec<RunRecord> = Vec::with_capacity(run_count);

    for _ in 0..run_count {
        let mut executor = rlm::router_executor::RlmExecutor::new(
            content.to_owned(),
            provider.clone(),
            model.to_owned(),
        )
        .with_temperature(run_temperature)
        .with_verbose(args.verbose);
        let mut result = executor.analyze(&args.query).await?;
        if result.sub_queries.is_empty() {
            result.sub_queries.push(rlm::SubQuery {
                query: args.query.clone(),
                context_slice: if args.file.len() == 1 {
                    Some(args.file[0].to_string_lossy().to_string())
                } else {
                    None
                },
                response: result.answer.clone(),
                tokens_used: result.stats.output_tokens,
            });
        }
        runs.push((
            result,
            executor.trace_steps().to_vec(),
            executor.context_trace_summary(),
        ));
    }

    Ok(runs)
}
