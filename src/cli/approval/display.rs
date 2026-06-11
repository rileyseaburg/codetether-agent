use super::{format, records};
use crate::approval::ApprovalStore;
use anyhow::{Result, anyhow};

pub(crate) fn list(store: &ApprovalStore) -> Result<()> {
    let records = records::load(store)?;
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

pub(crate) fn show(store: &ApprovalStore, id: &str) -> Result<()> {
    let record = records::load(store)?
        .into_iter()
        .find(|record| record.request.id == id)
        .ok_or_else(|| anyhow!("approval request not found"))?;
    format::print_request(&record.request, record.status());
    if let Some(decision) = record.decision {
        format::print_decision(&decision);
    }
    Ok(())
}
