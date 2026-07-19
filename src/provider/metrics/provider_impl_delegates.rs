//! Small synchronous delegations used by the metrics provider trait impl.

macro_rules! delegate_metrics_provider_basics {
    () => {
        fn name(&self) -> &str {
            self.inner.name()
        }

        fn supports_structured_streaming(&self) -> bool {
            self.inner.supports_structured_streaming()
        }

        fn begin_stream_recovery(&self, session_id: &str) {
            self.inner.begin_stream_recovery(session_id);
        }

        fn try_stream_fallback(&self, request: &CompletionRequest, session_id: &str) -> bool {
            self.inner.try_stream_fallback(request, session_id)
        }
    };
}

pub(super) use delegate_metrics_provider_basics;
