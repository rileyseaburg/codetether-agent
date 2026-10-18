//! dispatch context view over the shared session fields.
/// Internal context-source binding for application-local sessions.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_context_dispatch {
    ($name:ident) => {
        impl $crate::__private::context_dispatch::Source for $name {
            fn saved_policy(&self) -> $crate::__private::DerivePolicy {
                self.metadata.context_policy
            }
            fn constraint_prefix(&self) -> ::std::vec::Vec<::std::string::String> {
                $crate::__private::store::constraints::prefix(self)
            }
            fn message_offset(&self) -> usize {
                $name::message_offset(self)
            }
        }
    };
}
