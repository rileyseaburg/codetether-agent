//! Existing policy and preparation callbacks for context requests.
/// Internal source binding for an application-local session.
#[doc(hidden)]
#[macro_export]
macro_rules! __context_request_session_source {
    ($session:ty, $effective:path, $prepare:path) => {
        impl $crate::Source for $session {
            fn session_id(&self) -> &str {
                &self.id
            }
            fn effective_policy(&self, model: &str) -> $crate::session_binding::DerivePolicy {
                $effective(self, model)
            }
            fn prepare(
                &self,
                provider: ::std::sync::Arc<dyn $crate::session_binding::Provider>,
                model: &str,
            ) {
                $prepare(self, provider, model);
            }
        }
    };
}
