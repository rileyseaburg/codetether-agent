//! Frontend wrapper for shared ordered submission.
/// Declare the frontend loop submission entry with its concrete dispatcher.
/// # Examples
/// See the frontend loop-submit module for concrete channel and host types.
#[macro_export]
macro_rules! declare_frontend_loop_submit {
    ($request:path, $notice:path, $event:path, $active:path, $host:path $(,)?) => {
        use ::tokio::sync::mpsc;
        use $active as ActiveCancel;
        use $event as SessionEvent;
        use $host as Native;
        use $notice as SessionNotice;
        use $request as PromptRequest;
        pub(super) async fn submit(
            request: PromptRequest,
            cancel: &ActiveCancel,
            event_tx: &mpsc::Sender<SessionEvent>,
            notice_tx: &mpsc::Sender<SessionNotice>,
        ) -> bool {
            let mut host = Native {
                cancel: cancel.clone(),
                event_tx: event_tx.clone(),
                notice_tx: notice_tx.clone(),
            };
            $crate::dispatch::submit(&mut host, request).await
        }
    };
}
