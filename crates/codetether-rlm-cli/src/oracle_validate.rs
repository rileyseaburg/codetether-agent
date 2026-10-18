//! Deterministic or consensus validation of completed runs.
use crate::RunRecord;
use codetether_cli_options::RlmArgs;
use codetether_rlm as rlm;
pub(super) fn validate(
    args: &RlmArgs,
    runs: &[RunRecord],
    content: &str,
    source_path: Option<&str>,
) -> rlm::OracleResult {
    let validator = rlm::TraceValidator::new().with_consensus_threshold(args.consensus_threshold);
    let run_results: Vec<rlm::RlmAnalysisResult> = runs.iter().map(|(r, _, _)| r.clone()).collect();
    let trace_steps = runs.first().map(|(_, t, _)| t.clone()).unwrap_or_default();
    let oracle_result = if run_results.len() > 1 {
        validator.validate_with_consensus(
            &run_results,
            &content,
            source_path,
            None,
            Some(trace_steps),
        )
    } else {
        validator.validate(
            &run_results[0],
            &content,
            source_path.as_deref(),
            None,
            Some(trace_steps),
        )
    };

    oracle_result
}
