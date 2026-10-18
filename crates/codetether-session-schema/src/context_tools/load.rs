//! load tool storage adapter over the shared session schema.
/// Internal context tool binding for application-local session types.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_context_tool_load {
    ($name:ident, $bus:ty, $resolver:path) => {
        #[$crate::__private::async_trait]
        impl $crate::__private::context_loader::Source for $name {
            async fn load_tail(id: &str, window: usize) -> $crate::__private::anyhow::Result<Self> {
                $name::load_tail(id, window)
                    .await
                    .map(|loaded| loaded.session)
            }
            async fn last_for_directory_tail(
                workspace: ::std::option::Option<&::std::path::Path>,
                window: usize,
            ) -> $crate::__private::anyhow::Result<Self> {
                $name::last_for_directory_tail(workspace, window)
                    .await
                    .map(|loaded| loaded.session)
            }
            async fn last_for_directory(
                workspace: ::std::option::Option<&::std::path::Path>,
            ) -> $crate::__private::anyhow::Result<Self> {
                $name::last_for_directory(workspace).await
            }
        }
    };
}
