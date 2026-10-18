//! proactive context view over the shared session fields.
/// Internal context-source binding for application-local sessions.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_context_proactive {
    ($name:ident) => {
        impl $crate::__private::proactive_context::Source for $name {
            fn id(&self) -> &str {
                &self.id
            }
            fn messages(&self) -> &[$crate::__private::Message] {
                &self.messages
            }
            fn message_offset(&self) -> usize {
                $name::message_offset(self)
            }
            fn rlm_config(&self) -> &$crate::__private::RlmConfig {
                &self.metadata.rlm
            }
            fn subcall_provider(
                &self,
            ) -> ::std::option::Option<&::std::sync::Arc<dyn $crate::__private::Provider>> {
                self.metadata.subcall_provider.as_ref()
            }
            fn subcall_model_name(&self) -> ::std::option::Option<&str> {
                self.metadata.subcall_model_name.as_deref()
            }
        }
    };
}
