use super::Transfer;
/// Reserve steering state and transfer one owned request to the runtime.
/// # Arguments
/// Host supplies runtime/slot operations; request owns the session being moved.
/// # Returns
/// Completes with ownership transferred or restored and status updated.
/// # Examples
/// ```ignore
/// submit(&mut native, request).await;
/// ```
pub async fn submit<H: Transfer>(host: &mut H, request: H::Request) {
    if !host.activate(H::request_id(&request)) {
        host.restore(H::into_session(request));
        host.status("Session runtime is busy");
        return;
    }
    if let Err(request) = host.submit(request).await {
        host.clear_steering();
        host.restore(H::into_session(request));
        host.status("Session runtime is unavailable");
    }
}
