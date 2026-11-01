//! Stream admission survives headers but releases on terminal events and cancellation.
use super::super::{config::Config, gate::Gate};
use super::*;
use std::time::Duration;
fn gate() -> std::sync::Arc<Gate> {
    Gate::new(Config {
        output: 4096,
        waiting: 0,
        wait: Duration::from_secs(1),
        generation: Duration::from_millis(60),
    })
}
async fn available(gate: &std::sync::Arc<Gate>) {
    for _ in 0..100 {
        if gate.acquire().await.is_ok() {
            return;
        }
        tokio::task::yield_now().await;
        tokio::time::sleep(Duration::from_millis(2)).await;
    }
    panic!("generation permit leaked");
}
mod deadlines;
mod lifecycle;
