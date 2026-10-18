//! reset context view over the shared session fields.
/// Internal context-source binding for application-local sessions.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_context_reset {
    ($name:ident) => {
        impl $crate::__private::reset_derivation::Source for $name {
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
