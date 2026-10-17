//! Keep canonical session input and durable gates live during delegated work.

use super::{ControlHandle, boundary, steering};
use crate::session::Session;
use anyhow::Result;

pub(crate) fn drain(session: &mut Session, control: &ControlHandle, closing: bool) -> usize {
    let before = session.messages.len();
    let count = if closing {
        steering::drain_or_close_into(session)
    } else {
        steering::drain_into(session)
    };
    for message in &session.messages[before..] {
        control.publish_input(message.clone());
    }
    count
}

pub(crate) async fn run<T>(
    session: &mut Session,
    control: &ControlHandle,
    future: impl std::future::Future<Output = Result<T>>,
) -> Result<T> {
    tokio::pin!(future);
    let mut tick = tokio::time::interval(std::time::Duration::from_millis(100));
    let mut stopping = None;
    let started = std::time::Instant::now();
    loop {
        tokio::select! {
            result = &mut future => {
                let accounting = super::goals::record_usage(&session.id, 0, started.elapsed()).await;
                if stopping.is_none() { stopping = accounting.err(); }
                if stopping.is_none() {
                    stopping = control.ready().await.err();
                }
                while drain(session, control, stopping.is_some() || result.is_err()) > 0 {}
                return match stopping { Some(error) => Err(error), None => result };
            }
            _ = tick.tick(), if stopping.is_none() => {
                if let Err(error) = boundary::ensure_running(&session.id).await {
                    stopping = Some(error);
                    control.cancel();
                } else { drain(session, control, false); }
            }
            _ = control.cancelled(), if stopping.is_none() => {
                stopping = Some(anyhow::anyhow!("Session execution cancelled"));
            }
        }
    }
}
