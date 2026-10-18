//! Check the already-started optional S3 sink for unexpected termination.
use super::*;

pub(super) async fn ensure_s3_sync_alive(
    handle: &mut Option<tokio::task::JoinHandle<Result<()>>>,
) -> Result<()> {
    let Some(inner) = handle.as_ref() else {
        anyhow::bail!("S3 sync task missing");
    };
    if !inner.is_finished() {
        return Ok(());
    }

    let finished = handle.take().expect("checked is_some");
    match finished.await {
        Ok(Ok(())) => {
            anyhow::bail!("S3 sync task exited unexpectedly");
        }
        Ok(Err(err)) => Err(anyhow::anyhow!("S3 sync task failed: {err:#}")),
        Err(join_err) => Err(anyhow::anyhow!("S3 sync task join failure: {join_err}")),
    }
}
