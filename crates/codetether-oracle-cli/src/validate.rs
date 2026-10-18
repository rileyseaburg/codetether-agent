//! Load inputs, validate the synthetic analysis and optionally persist the result.
use crate::{
    OracleValidateArgs,
    input::{load_payload, load_source},
};
use anyhow::Result;
use codetether_rlm::oracle::{OracleTraceStorage, TraceValidator};

pub(super) async fn execute_validate(args: OracleValidateArgs) -> Result<()> {
    let source = load_source(&args)?;
    let payload = load_payload(&args)?;
    let source_path = args.file.as_ref().map(|p| p.to_string_lossy().to_string());

    let synthetic = crate::synthetic::build(&args.query, &source, &payload, source_path.clone());

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

    crate::output::print(args.json, &oracle_result, persist)?;

    Ok(())
}
