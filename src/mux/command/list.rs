//! Named mux session listing.

use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
struct ListedSession {
    name: String,
    address: String,
    pid: u32,
    windows: usize,
    active: u64,
    reachable: bool,
}

pub(super) async fn run(json: bool) -> Result<()> {
    let mut listed = Vec::new();
    for record in crate::mux::registry::list().await? {
        let reachable = tokio::time::timeout(
            std::time::Duration::from_millis(300),
            crate::mux::client::probe(&record),
        )
        .await
        .is_ok_and(|result| result.is_ok());
        listed.push(ListedSession {
            name: record.name,
            address: record.address.to_string(),
            pid: record.pid,
            windows: record.state.windows.len(),
            active: record.state.active_window,
            reachable,
        });
    }
    if json {
        println!("{}", serde_json::to_string_pretty(&listed)?);
    } else if listed.is_empty() {
        println!("no mux sessions");
    } else {
        for item in listed {
            print_item(&item);
        }
    }
    Ok(())
}

fn print_item(item: &ListedSession) {
    let status = if item.reachable { "up" } else { "stale" };
    println!(
        "{}: {} windows (active {}) [{}] pid={} {}",
        item.name, item.windows, item.active, status, item.pid, item.address
    );
}
