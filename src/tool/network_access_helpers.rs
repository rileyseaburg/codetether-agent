//! No-redirect HTTP client construction for approval-bound network tools.

/// Build a client that never retargets a reviewed request through redirects.
pub(crate) fn no_redirect_client() -> anyhow::Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?)
}

#[cfg(test)]
#[path = "network_redirect_contract_tests.rs"]
mod tests;

pub(crate) fn no_redirect_client_with_timeout(
    timeout: std::time::Duration,
) -> anyhow::Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .timeout(timeout)
        .redirect(reqwest::redirect::Policy::none())
        .user_agent("CodeTether-Agent/1.0")
        .build()?)
}