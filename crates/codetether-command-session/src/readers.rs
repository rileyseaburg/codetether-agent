//! Bounded asynchronous forwarding of child stdout and stderr.

use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::sync::mpsc;

use super::types::CommandInput;

const CHUNK_BYTES: usize = 8 * 1024;
const CHANNEL_CHUNKS: usize = 128;

pub(super) fn start(
    stdout: Option<tokio::process::ChildStdout>,
    stderr: Option<tokio::process::ChildStderr>,
) -> mpsc::Receiver<Vec<u8>> {
    let (sender, receiver) = mpsc::channel(CHANNEL_CHUNKS);
    if let Some(stdout) = stdout {
        spawn(stdout, sender.clone());
    }
    if let Some(stderr) = stderr {
        spawn(stderr, sender.clone());
    }
    drop(sender);
    receiver
}

pub(super) fn terminal(
    master: std::fs::File,
) -> std::io::Result<(CommandInput, mpsc::Receiver<Vec<u8>>)> {
    let writer = tokio::fs::File::from_std(master.try_clone()?);
    let reader = tokio::fs::File::from_std(master);
    let (sender, receiver) = mpsc::channel(CHANNEL_CHUNKS);
    spawn(reader, sender);
    Ok((Box::pin(writer), receiver))
}

fn spawn(mut stream: impl AsyncRead + Unpin + Send + 'static, sender: mpsc::Sender<Vec<u8>>) {
    tokio::spawn(async move {
        let mut bytes = vec![0; CHUNK_BYTES];
        loop {
            let Ok(count) = stream.read(&mut bytes).await else {
                break;
            };
            if count == 0 || sender.send(bytes[..count].to_vec()).await.is_err() {
                break;
            }
        }
    });
}
