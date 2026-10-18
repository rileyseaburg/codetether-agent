//! Existing derivation and recall callbacks for context requests.
/// Internal asynchronous request host binding.
#[doc(hidden)]
#[macro_export]
macro_rules! __context_request_session_host {
    ($session:ty, $derive:path, $prefetch:path) => {
        #[$crate::session_binding::async_trait]
        impl $crate::Host for $session {
            async fn derive(
                &self,
                input: $crate::Derivation<'_>,
            ) -> $crate::session_binding::anyhow::Result<$crate::session_binding::DerivedContext>
            {
                $derive(
                    self,
                    input.provider,
                    input.model,
                    input.system_prompt,
                    input.tools,
                    None,
                    input.policy,
                    input.force_keep_last,
                )
                .await
            }
            async fn prefetch(&self) -> ::std::option::Option<$crate::session_binding::Message> {
                $prefetch(self).await
            }
        }
    };
}
