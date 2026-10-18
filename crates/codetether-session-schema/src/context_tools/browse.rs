//! browse tool storage adapter over the shared session schema.
/// Internal context tool binding for application-local session types.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_context_tool_browse {
    ($name:ident, $bus:ty, $resolver:path) => {
        #[$crate::__private::async_trait]
        impl $crate::__private::context_browse::BrowseSource for $name {
            fn id(&self) -> &str {
                &self.id
            }
            fn workspace(&self) -> ::std::option::Option<&::std::path::Path> {
                self.metadata.directory.as_deref()
            }
            fn message_count(&self) -> usize {
                $name::message_count(self)
            }
            async fn read_messages(
                id: &str,
                start: usize,
                limit: usize,
            ) -> $crate::__private::anyhow::Result<::std::vec::Vec<$crate::__private::Message>>
            {
                $name::read_messages(id, start, limit).await
            }
        }
    };
}
