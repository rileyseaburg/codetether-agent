//! legacy context view over the shared session fields.
/// Internal context-source binding for application-local sessions.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_context_legacy {
    ($name:ident) => {
        impl $crate::__private::context_legacy::Source for $name {
            fn messages(&self) -> &[$crate::__private::Message] {
                &self.messages
            }
            fn session_id(&self) -> &str {
                &self.id
            }
            fn rlm_config(&self) -> &$crate::__private::RlmConfig {
                &self.metadata.rlm
            }
        }
    };
}
