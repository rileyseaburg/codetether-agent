//! Move a returned session into the runtime, restoring rejected requests.
/// Implementation detail of [crate::declare_frontend_retry].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_retry_submission {
    ($slot:path, $registry:path, $runtime:path, $request:path) => {
        mod submission {
            use ::std::sync::Arc;
            use $registry as ProviderRegistry;
            use $request as PromptRequest;
            use $runtime as TuiSessionHandle;
            use $slot as SessionSlot;
            pub(super) async fn submit(
                slot: &mut SessionSlot,
                prompt: &str,
                runtime: &TuiSessionHandle,
                registry: &Arc<ProviderRegistry>,
            ) {
                let Some(session) = slot.take_for_prompt() else {
                    return;
                };
                let original_dir = session.metadata.directory.clone();
                let request = PromptRequest::new(
                    session,
                    prompt.to_string(),
                    Vec::new(),
                    Arc::clone(registry),
                    original_dir,
                    None,
                );
                if let Err(request) = runtime.submit(request).await {
                    slot.restore(request.session);
                }
            }
        }
    };
}
