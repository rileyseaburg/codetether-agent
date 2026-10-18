//! Oracle trace persistence and compatible failure receipt.
use codetether_rlm as rlm;
pub(super) async fn persist(
    oracle_result: &rlm::OracleResult,
) -> Option<rlm::OracleTracePersistResult> {
    let storage = rlm::OracleTraceStorage::from_env_or_vault().await;
    let persist = match storage.persist_result(oracle_result).await {
        Ok(p) => Some(p),
        Err(e) => Some(rlm::OracleTracePersistResult {
            verdict: oracle_result.to_record().verdict,
            spooled_path: String::new(),
            uploaded: false,
            remote_key: None,
            remote_url: None,
            pending_count: 0,
            warning: Some(format!("Failed to persist oracle record: {}", e)),
        }),
    };

    persist
}
