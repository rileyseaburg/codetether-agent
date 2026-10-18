//! Borrowed session metadata projection for compact headers.
/// Internal header binding for application-local session types.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_storage_header {
    ($name:ident) => {
        impl $crate::HeaderSource for $name {
            fn header(&self) -> $crate::__private::codec::HeaderView<'_> {
                $crate::__private::codec::HeaderView {
                    id: &self.id,
                    title: &self.title,
                    created_at: &self.created_at,
                    updated_at: &self.updated_at,
                    metadata: &self.metadata,
                    agent: &self.agent,
                    usage: &self.usage,
                }
            }
        }
    };
}
