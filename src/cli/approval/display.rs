use super::{format, json, records};
use crate::approval::ApprovalStore;
use anyhow::{Result, anyhow};

/// Print every approval request.
///
/// # Arguments
///
/// * `store` — Approval store to read.
/// * `as_json` — Emit a JSON array instead of the aligned text table.
///
/// # Errors
///
/// Returns an error when the store cannot be read or serialization fails.
pub(crate) fn list(store: &ApprovalStore, as_json: bool) -> Result<()> {
    let records = records::load(store)?;
    if as_json {
        // Always valid JSON, including `[]`, so a caller never has to special-case
        // the empty store the way the text output's message forces.
        println!("{}", json::list(&records)?);
        return Ok(());
    }
    if records.is_empty() {
        println!("No approval requests.");
        return Ok(());
    }
    println!(
        "{:<36} {:<8} {:<14} {:<10} {}",
        "ID", "STATUS", "TOOL", "ACTION", "RESOURCE"
    );
    for record in records {
        println!(
            "{:<36} {:<8} {:<14} {:<10} {}",
            record.request.id,
            format::status_label(record.status()),
            record.request.tool,
            record.request.action,
            record.request.resource
        );
    }
    Ok(())
}

/// Print one approval request and its decision.
///
/// # Arguments
///
/// * `store` — Approval store to read.
/// * `id` — Request id to show.
/// * `as_json` — Emit a JSON object instead of `key: value` lines.
///
/// # Errors
///
/// Returns an error when the request is absent, the store cannot be read, or
/// serialization fails.
pub(crate) fn show(store: &ApprovalStore, id: &str, as_json: bool) -> Result<()> {
    let record = records::load(store)?
        .into_iter()
        .find(|record| record.request.id == id)
        .ok_or_else(|| anyhow!("approval request not found"))?;
    if as_json {
        println!("{}", json::show(&record)?);
        return Ok(());
    }
    format::print_request(&record.request, record.status());
    if let Some(decision) = record.decision {
        format::print_decision(&decision);
    }
    Ok(())
}
