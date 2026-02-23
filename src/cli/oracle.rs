//! Oracle command handlers.

use super::{OracleArgs, OracleCommand, OracleSyncArgs, OracleValidateArgs};
use crate::rlm::{
    OracleResult, OracleTraceStorage, RlmAnalysisResult, RlmChunker, RlmStats, SubQuery,
    TraceValidator,
};
use anyhow::{Context, Result};
use serde_json::json;
use std::io::Read;

pub async fn execute(args: OracleArgs) -> Result<()> {
    match args.command {
        OracleCommand::Validate(v) => execute_validate(v).await,
        OracleCommand::Sync(s) => execute_sync(s).await,
    }
}

async fn execute_validate(args: OracleValidateArgs) -> Result<()> {
    let source = load_source(&args)?;
    let payload = load_payload(&args)?;
    let source_path = args.file.as_ref().map(|p| p.to_string_lossy().to_string());

    let synthetic = RlmAnalysisResult {
        answer: payload.clone(),
        iterations: 1,
        sub_queries: vec![SubQuery {
            query: args.query.clone(),
            context_slice: source_path.clone(),
            response: payload.clone(),
            tokens_used: 0,
        }],
        stats: RlmStats {
            input_tokens: RlmChunker::estimate_tokens(&source),
            output_tokens: RlmChunker::estimate_tokens(&payload),
            iterations: 1,
            subcalls: 0,
            elapsed_ms: 0,
            compression_ratio: 1.0,
        },
    };

    let validator = TraceValidator::new();
    let oracle_result = validator.validate(
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
    }

    Ok(())
}

async fn execute_sync(args: OracleSyncArgs) -> Result<()> {
    let storage = OracleTraceStorage::from_env_or_vault().await;
    let stats = storage.sync_pending().await?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&stats)?);
    } else {
        println!(
            "synced={} retained={} failed={} pending_after={}",
            stats.uploaded, stats.retained, stats.failed, stats.pending_after
        );
    }

    Ok(())
}

fn load_source(args: &OracleValidateArgs) -> Result<String> {
    match (&args.file, &args.content) {
        (Some(path), None) => std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read source file {}", path.display())),
        (None, Some(content)) if content == "-" => {
            let mut stdin_content = String::new();
            std::io::stdin().read_to_string(&mut stdin_content)?;
            Ok(stdin_content)
        }
        (None, Some(content)) => Ok(content.clone()),
        (Some(_), Some(_)) => {
            anyhow::bail!("Provide either --file or --content, not both");
        }
        (None, None) => {
            anyhow::bail!("Provide --file <path> or --content <text|->");
        }
    }
}

fn load_payload(args: &OracleValidateArgs) -> Result<String> {
    match (&args.payload, &args.payload_file) {
        (Some(payload), None) => Ok(payload.clone()),
        (None, Some(path)) => std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read payload file {}", path.display())),
        (Some(_), Some(_)) => {
            anyhow::bail!("Provide either --payload or --payload-file, not both");
        }
        (None, None) => {
            anyhow::bail!("Provide --payload <json> or --payload-file <path>");
        }
    }
}

fn oracle_json(result: &OracleResult) -> serde_json::Value {
    match result {
        OracleResult::Golden(trace) => json!({
            "status": "golden",
            "verdict": trace.verdict,
            "verification_method": format!("{:?}", trace.verification_method),
            "trace_id": trace.trace_id,
        }),
        OracleResult::Consensus {
            trace,
            agreement_ratio,
        } => json!({
            "status": "consensus",
            "agreement_ratio": agreement_ratio,
            "verdict": trace.verdict,
            "verification_method": format!("{:?}", trace.verification_method),
            "trace_id": trace.trace_id,
        }),
        OracleResult::Unverified { reason, trace } => json!({
            "status": "unverified",
            "reason": reason,
            "verdict": trace.verdict,
            "verification_method": format!("{:?}", trace.verification_method),
            "trace_id": trace.trace_id,
        }),
        OracleResult::Failed {
            reason,
            diff,
            trace,
        } => json!({
            "status": "failed",
            "reason": reason,
            "diff": diff,
            "verdict": trace.verdict,
            "verification_method": format!("{:?}", trace.verification_method),
            "trace_id": trace.trace_id,
        }),
    }
}

fn oracle_status_line(result: &OracleResult) -> String {
    match result {
        OracleResult::Golden(_) => {
            "[oracle: golden ✓] deterministic verification passed".to_string()
        }
        OracleResult::Consensus {
            agreement_ratio, ..
        } => format!(
            "[oracle: consensus ✓] semantic agreement {:.1}%",
            agreement_ratio * 100.0
        ),
        OracleResult::Unverified { reason, .. } => {
            format!("[oracle: unverified —] {}", reason)
        }
        OracleResult::Failed { reason, .. } => format!("[oracle: failed ✗] {}", reason),
    }
}
