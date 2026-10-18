use super::process::PeerProcess;
use std::time::Duration;

pub struct Routes {
    pub beta: String,
}

pub async fn reciprocal(
    alpha: &mut PeerProcess,
    beta: &mut PeerProcess,
    alpha_name: &str,
    beta_name: &str,
) -> Routes {
    let result = tokio::time::timeout(Duration::from_secs(20), async {
        loop {
            alpha.assert_running();
            beta.assert_running();
            let alpha_log = alpha.log();
            let beta_log = beta.log();
            if let (Some(beta_url), Some(_alpha_url)) = (
                introduced_endpoint(&alpha_log, beta_name),
                introduced_endpoint(&beta_log, alpha_name),
            ) {
                return Routes { beta: beta_url };
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await;
    result.unwrap_or_else(|_| {
        panic!(
            "reciprocal mDNS proof timed out\nALPHA:\n{}\nBETA:\n{}",
            alpha.log(),
            beta.log()
        )
    })
}

fn introduced_endpoint(log: &str, peer: &str) -> Option<String> {
    let discovery = log.lines().find(|line| {
        line.contains("Discovered A2A peer") && line.contains(&format!("peer_name={peer}"))
    })?;
    let endpoint = discovery
        .split_whitespace()
        .find_map(|field| field.strip_prefix("endpoint="))?
        .to_string();
    log.contains(&format!("Auto-intro message sent peer={endpoint}"))
        .then_some(endpoint)
}
