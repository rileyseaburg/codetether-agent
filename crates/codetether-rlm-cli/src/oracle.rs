//! Optional oracle verification after all analysis runs complete.
use crate::RunRecord;
use codetether_cli_options::RlmArgs;
pub(super) async fn evaluate(
    args: &RlmArgs,
    runs: &[RunRecord],
    content: &str,
    source_path: Option<&str>,
) -> anyhow::Result<(String, serde_json::Value)> {
    let oracle_enabled = !args.no_oracle_verify;
    let mut oracle_json: serde_json::Value = serde_json::json!({
        "status": "disabled",
        "reason": "disabled via --no-oracle-verify",
    });
    let mut oracle_status = "[oracle: disabled —] verification disabled".to_string();

    if oracle_enabled {
        let oracle_result = crate::oracle_validate::validate(args, runs, content, source_path);
        let split = crate::oracle_split::write(args, &oracle_result)?;
        let persist = crate::oracle_persist::persist(&oracle_result).await;
        oracle_status = crate::oracle_status_line(&oracle_result);
        oracle_json = crate::oracle_json_value(&oracle_result, split, persist);
    }
    Ok((oracle_status, oracle_json))
}
