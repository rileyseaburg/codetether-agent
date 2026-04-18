use crate::browser::{
    BrowserError, BrowserOutput,
    output::Ack,
    session::{BrowserSession, SessionMode, SessionRuntime},
};
use std::time::Duration;

pub(super) async fn run(session: &BrowserSession) -> Result<BrowserOutput, BrowserError> {
    let runtime = {
        let mut slot = session.inner.runtime.lock().await;
        slot.take()
    };
    if let Some(runtime) = runtime {
        shutdown(runtime).await?;
    }
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

pub(super) async fn shutdown(mut runtime: SessionRuntime) -> Result<(), BrowserError> {
    if runtime.mode == SessionMode::Launch {
        let mut browser = runtime.browser.lock().await;
        let _ = browser.close().await;
        let _ = browser.wait().await;
    }
    let _ = runtime.shutdown.send(true);
    if tokio::time::timeout(Duration::from_secs(5), &mut runtime.handler_task)
        .await
        .is_err()
    {
        runtime.handler_task.abort();
        let _ = runtime.handler_task.await;
    }
    Ok(())
}
