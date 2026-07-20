//! Asynchronous child output capture into the task replay buffer.

use std::sync::Arc;

use tokio::io::{AsyncRead, AsyncReadExt};

use super::entry::AgentTask;

pub(super) fn start<R>(mut reader: R, task: Arc<AgentTask>) -> tokio::task::JoinHandle<()>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        let mut buffer = vec![0; 8192];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) | Err(_) => return,
                Ok(count) => task.append(&buffer[..count]),
            }
        }
    })
}
