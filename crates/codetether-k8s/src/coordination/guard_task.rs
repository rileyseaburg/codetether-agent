//! Background renewal loop for an acquired mutation lease.

use tokio::sync::oneshot;

use super::renew;
use super::repository::LeaseRepository;

pub fn spawn(
    repo: LeaseRepository,
    name: String,
    holder: String,
    ttl: i32,
    mut stopped: oneshot::Receiver<()>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let period = std::time::Duration::from_secs((ttl / 3).max(1) as u64);
        let mut tick = tokio::time::interval(period);
        loop {
            tokio::select! {
                _ = &mut stopped => {
                    let _ = renew::release(&repo, &name, &holder).await;
                    break;
                }
                _ = tick.tick() => match renew::renew(&repo, &name, &holder).await {
                    Ok(true) => {}
                    Ok(false) | Err(_) => break,
                }
            }
        }
    })
}
