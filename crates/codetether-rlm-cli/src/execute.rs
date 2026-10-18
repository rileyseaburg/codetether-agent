//! RLM CLI phase ordering, independent of provider initialization.
use crate::{ProviderSource, detect_rlm_content_type, gather_rlm_content};
use codetether_cli_options::RlmArgs;
/// Execute validation, analysis, verification, persistence and output in order.
/// # Arguments
/// * `args` - Parsed RLM command options.
/// # Returns
/// Unit after the requested report is printed.
/// # Errors
/// Propagates input, provider, analysis, split-output and serialization failures.
/// # Examples
/// ```no_run
/// # async fn example<P: codetether_rlm_cli::ProviderSource>(args: codetether_cli_options::RlmArgs) {
/// let _ = codetether_rlm_cli::execute::<P>(args).await;
/// # }
/// ```
pub async fn execute<P: ProviderSource>(args: RlmArgs) -> anyhow::Result<()> {
    crate::validation::validate(&args)?;
    let content = gather_rlm_content(&args)?;
    let input_tokens = codetether_rlm::RlmChunker::estimate_tokens(&content);
    let content_type = detect_rlm_content_type(&args, &content);
    tracing::info!(input_tokens, "RLM: Analyzing content");
    let (provider, model, provider_name) = P::select(args.model.as_deref()).await?;
    tracing::info!(provider = %provider_name, model = %model, "RLM provider resolved");
    let runs = crate::analysis::run(&args, &content, &provider, &model).await?;
    let result = runs
        .first()
        .map(|(r, _, _)| r.clone())
        .ok_or_else(|| anyhow::anyhow!("No RLM runs executed"))?;
    let source_path = if args.file.len() == 1 {
        Some(args.file[0].to_string_lossy().to_string())
    } else {
        None
    };

    let (oracle_status, oracle_json) =
        crate::oracle::evaluate(&args, &runs, &content, source_path.as_deref()).await?;
    let output = crate::output::Output {
        args: &args,
        runs: &runs,
        provider_name: &provider_name,
        model: &model,
        content_type: &content_type,
        input_tokens,
        result: &result,
        oracle_status: &oracle_status,
        oracle_json: &oracle_json,
    };
    if args.json {
        crate::output_json::write(&output)
    } else {
        crate::output_text::write(&output)
    }
}
