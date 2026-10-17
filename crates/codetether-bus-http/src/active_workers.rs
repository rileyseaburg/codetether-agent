//! Filter discovery records to active workers advertising an interface.
pub(super) fn active_workers(workers: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
    workers
        .into_iter()
        .filter(|worker| {
            let status = worker
                .get("status")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            status == "active"
                && worker
                    .get("interfaces")
                    .and_then(|value| value.as_object())
                    .map(|value| !value.is_empty())
                    .unwrap_or(false)
        })
        .collect::<Vec<_>>()
}
