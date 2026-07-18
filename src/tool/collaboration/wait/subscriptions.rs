//! Await status changes across multiple child watch channels.

use super::targets::Receiver;
use crate::tool::agent::collaboration_runtime::thread_status::ThreadStatus;
use futures::{FutureExt, StreamExt, stream::FuturesUnordered};
use serde_json::{Map, Value, to_value};
use tokio::time::Instant;

pub(super) async fn first_final(
    receivers: Vec<(String, Receiver)>,
    deadline: Instant,
) -> Map<String, Value> {
    let mut futures = receivers
        .into_iter()
        .map(|(id, receiver)| wait_one(id, receiver))
        .collect::<FuturesUnordered<_>>();
    let mut settled = Map::new();
    if let Ok(Some(Some((id, status)))) = tokio::time::timeout_at(deadline, futures.next()).await {
        insert(&mut settled, id, status);
        while let Some(Some(Some((id, status)))) = futures.next().now_or_never() {
            insert(&mut settled, id, status);
        }
    }
    settled
}

async fn wait_one(id: String, mut receiver: Receiver) -> Option<(String, ThreadStatus)> {
    loop {
        if receiver.changed().await.is_err() {
            let status = receiver.borrow().clone();
            return status.is_final().then_some((id, status));
        }
        let status = receiver.borrow().clone();
        if status.is_final() {
            return Some((id, status));
        }
    }
}

fn insert(output: &mut Map<String, Value>, id: String, status: ThreadStatus) {
    output.insert(id, to_value(status).unwrap_or(Value::Null));
}

#[cfg(test)]
#[path = "subscriptions_tests.rs"]
mod tests;
