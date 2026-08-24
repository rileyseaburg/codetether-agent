//! HTTP client policy for search-provider requests.

use std::time::Duration;

pub(super) fn build() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .redirect(reqwest::redirect::Policy::none())
        .user_agent("CodeTether-Agent/1.0")
        .build()
        .expect("Failed to build HTTP client")
}

#[cfg(test)]
#[path = "websearch_client_tests.rs"]
mod tests;
