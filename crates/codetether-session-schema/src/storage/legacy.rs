//! Legacy implementation for the shared session schema.
/// Internal storage binding for application-local session types.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_storage_legacy {
    ($name:ident) => {
        impl $crate::__private::store::legacy::Legacy for $name {
            fn legacy_header(&self) -> $crate::__private::anyhow::Result<::std::string::String> {
                $crate::__private::codec::legacy_header($crate::header(self))
            }
            fn summary_index(&self) -> &$crate::__private::SummaryIndex {
                &self.summary_index
            }
            fn is_at_least_as_new_as(&self, incoming: &Self) -> bool {
                self.updated_at >= incoming.updated_at
            }
            fn replace_state(&mut self, state: $crate::__private::State) {
                self.storage = state;
            }
        }
    };
}
