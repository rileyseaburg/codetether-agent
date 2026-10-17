//! Provider history aggregation.

use super::*;

/// Group `requests` by provider and fold each group into a [`ProviderSnapshot`].
pub(super) fn aggregate_by_provider(requests: Vec<ProviderRequestRecord>) -> Vec<ProviderSnapshot> {
    let mut by_provider: HashMap<String, Vec<ProviderRequestRecord>> = HashMap::new();
    for req in requests {
        by_provider
            .entry(req.provider.clone())
            .or_default()
            .push(req);
    }
    by_provider
        .into_iter()
        .filter(|(_, reqs)| !reqs.is_empty())
        .map(|(provider, reqs)| snapshot_for(provider, &reqs))
        .collect()
}
