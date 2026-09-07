//! Background lease renewal with bounded retries and explicit reauthentication errors.

use super::{Clients, api, schedule};
use std::{sync::Weak, time::Duration};

pub(super) async fn maintain(clients: Weak<Clients>) {
    let mut delay = Duration::ZERO;
    let mut retry_seconds = 1;
    let mut initial = true;
    loop {
        tokio::time::sleep(delay).await;
        let Some(client) = clients.upgrade().and_then(|clients| clients.read().clone()) else {
            return;
        };
        let lease = if initial {
            api::discover(&client).await
        } else {
            api::renew(&client).await
        };
        let discovery = initial;
        initial = false;
        match lease {
            Ok(lease) => {
                tracing::info!(
                    ttl_seconds = lease.ttl,
                    renewable = lease.renewable,
                    discovery,
                    "Vault token lease observed; automatic renewal active for renewable tokens"
                );
                let Some(next) = schedule::next(lease) else {
                    return;
                };
                delay = next;
                retry_seconds = 1;
            }
            Err(error) => {
                tracing::warn!(failure = ?error, "Vault token lease maintenance failed");
                if error.terminal() {
                    tracing::warn!(
                        "Vault renewal was rejected: token may be expired, revoked, at its maximum TTL, or lack renew-self permission; reauthenticate with a renewable token"
                    );
                    return;
                }
                delay = Duration::from_secs(retry_seconds);
                retry_seconds = (retry_seconds * 2).min(30);
            }
        }
    }
}
