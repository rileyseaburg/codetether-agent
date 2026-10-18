//! Deliver a decoded output chunk to the frontend subscriber.
use super::MuxLiveOutput;
use tokio::sync::mpsc::Sender;

pub(super) async fn send(
    sender: &Sender<MuxLiveOutput>,
    name: &str,
    data: Vec<u8>,
    offset: u64,
    running: bool,
) -> anyhow::Result<()> {
    sender
        .send(MuxLiveOutput {
            session: name.into(),
            data: String::from_utf8_lossy(&data).into_owned(),
            offset,
            running,
        })
        .await?;
    Ok(())
}
