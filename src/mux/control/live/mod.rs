//! Native fan-in for event-driven output from every registered mux server.

mod connection;
mod session;
mod types;

pub(crate) use types::MuxLiveOutput;

pub(crate) fn subscribe_live_output() -> tokio::sync::mpsc::Receiver<MuxLiveOutput> {
    let (sender, receiver) = tokio::sync::mpsc::channel(256);
    tokio::spawn(async move {
        let Ok(records) = crate::mux::registry::list().await else {
            return;
        };
        for record in records {
            tokio::spawn(session::follow(record.name, sender.clone()));
        }
    });
    receiver
}
