//! Small synchronous delegations used by the metrics provider trait impl.

macro_rules! delegate_metrics_provider_basics {
    () => {
        fn name(&self) -> &str {
            self.inner.name()
        }

        fn supports_structured_streaming(&self) -> bool {
            self.inner.supports_structured_streaming()
        }
    };
}

pub(super) use delegate_metrics_provider_basics;
