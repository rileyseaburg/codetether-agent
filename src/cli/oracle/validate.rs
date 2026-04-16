//! Oracle validate subcommand — loads source/payload, validates, and displays results.

use super::format::{oracle_json, oracle_status_line};
use crate::rlm::{
    OracleResult, OracleTracePersistResult, OracleTraceStorage, RlmAnalysisResult, RlmChunker,
    RlmStats, SubQuery, TraceValidator,
};
use anyhow::{Context, Result};
use serde_json::json;
use std::io::Read;

/// Execute the validate subcommand.
pub async fn execute_validate(args: super::OracleValidateArgs) -> Result<()> {
    let source = load_source(&args)?;
    let payload = load_payload(&args)?;
    let source_path = args.file.as_ref().map(|p| p.to_string_lossy().to_string());

    let synthetic = build_synthetic(&args, &source, &payload, &source_path);
    let oracle_result = TraceValidator::new().validate(
        &synthetic,
        &source,
        source_path.as_deref(),
        None,
        Some(Vec::new()),
    );

    let persist = if args.persist {
        let storage = OracleTraceStorage::from_env_or_vault().await;
        Some(storage.persist_result(&oracle_result).await?)
    } else {
        None
    };

    if args.json {
        let out = json!({
            "oracle": oracle_json(&oracle_result),
            "record": oracle_result.to_record(),
            "persist": persist,
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("{}", oracle_status_line(&oracle_result));
        if let Some(ref p) = persist {
            print_persist_info(p);
        }
    }
    Ok(())
}

/// Build a synthetic RLM analysis result from validate args.
fn build_synthetic(
    args: &super::OracleValidateArgs,
    source: &str,
    payload: &str,
    source_path: &Option<String>,
) -> RlmAnalysisResult {
    RlmAnalysisResult {
        answer: payload.to_string(),
        iterations: 1,
        sub_queries: vec![SubQuery {
            query: args.query.clone(),
            context_slice: source_path.clone(),
            response: payload.to_string(),
            tokens_used: 0,
        }],
        stats: RlmStats {
            input_tokens: RlmChunker::estimate_tokens(source),
            output_tokens: RlmChunker::estimate_tokens(payload),
            iterations: 1,
            subcalls: 0,
            elapsed_ms: 0,
            compression_ratio: 1.0,
        },
    }
}

/// Print human-readable persist info.
fn print_persist_info(p: &OracleTracePersistResult) {
    if p.uploaded {
        println!(
            "[persist: uploaded ✓] {}",
            p.remote_url.clone().unwrap_or_default()
        );
    } else {
        println!("[persist: spooled —] {}", p.spooled_path);
    }
    if let Some(ref w) = p.warning {
        println!("warning: {}", w);
    }
}

/// Load source content from file path, stdin, or inline content.
fn load_source(args: &super::OracleValidateArgs) -> Result<String> {
    match (&args.file, &args.content) {
        (Some(path), None) => std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read source file {}", path.display())),
        (None, Some(content)) if content == "-" => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            Ok(buf)
        }
        (None, Some(content)) => Ok(content.clone()),
        (Some(_), Some(_)) => anyhow::bail!("Provide either --file or --content, not both"),
        (None, None) => anyhow::bail!("Provide --file <path> or --content <text|->"),
    }
}

/// Load expected payload from inline argument or file path.
fn load_payload(args: &super::OracleValidateArgs) -> Result<String> {
    match (&args.payload, &args.payload_file) {
        (Some(payload), None) => Ok(payload.clone()),
        (None, Some(path)) => std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read payload file {}", path.display())),
        (Some(_), Some(_)) => {
            anyhow::bail!("Provide either --payload or --payload-file, not both")
        }
        (None, None) => anyhow::bail!("Provide --payload <json> or --payload-file <path>"),
    }
}
