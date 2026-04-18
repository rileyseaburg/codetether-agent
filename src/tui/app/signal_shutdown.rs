use tokio::sync::mpsc;

pub(super) fn spawn_shutdown_listener() -> mpsc::Receiver<()> {
    let (tx, rx) = mpsc::channel(1);
    tokio::spawn(async move {
        wait_for_shutdown_signal().await;
        let _ = tx.send(()).await;
    });
    rx
}

#[cfg(not(unix))]
async fn wait_for_shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

#[cfg(unix)]
async fn wait_for_shutdown_signal() {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = wait_for_unix(tokio::signal::unix::SignalKind::terminate()) => {}
        _ = wait_for_unix(tokio::signal::unix::SignalKind::hangup()) => {}
    }
}

#[cfg(unix)]
async fn wait_for_unix(kind: tokio::signal::unix::SignalKind) {
    use tokio::signal::unix::signal;

    if let Ok(mut stream) = signal(kind) {
        let _ = stream.recv().await;
    } else {
        std::future::pending::<()>().await;
    }
}
