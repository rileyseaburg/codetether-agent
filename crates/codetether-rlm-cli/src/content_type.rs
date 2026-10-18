//! RLM CLI content type handling.
use codetether_rlm as rlm;

/// Project RLM CLI content_type information.
/// # Arguments
/// * `args` — Requested content hint.
/// * `content` — Input for automatic type detection.
/// # Returns
/// Requested or automatically detected content type.
/// # Examples
/// ```no_run
/// # fn example(args: &codetether_cli_options::RlmArgs, content: &str) {
/// let _ = codetether_rlm_cli::detect_rlm_content_type(args, content);
/// # }
/// ```
pub fn detect_rlm_content_type(
    args: &codetether_cli_options::RlmArgs,
    content: &str,
) -> rlm::ContentType {
    if args.content_type == "auto" {
        return rlm::RlmChunker::detect_content_type(content);
    }

    match args.content_type.as_str() {
        "code" => rlm::ContentType::Code,
        "logs" => rlm::ContentType::Logs,
        "conversation" => rlm::ContentType::Conversation,
        "documents" => rlm::ContentType::Documents,
        _ => rlm::ContentType::Mixed,
    }
}
