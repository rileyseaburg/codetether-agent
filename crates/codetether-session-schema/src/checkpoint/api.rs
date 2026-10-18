//! Shared checkpoint api bindings for the local session schema.
/// Internal checkpoint implementation for the shared session declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_checkpoint_api {
    ($name:ident) => {
        impl $name {
            pub async fn save_run_checkpoint(
                &mut self,
                checkpoint: $crate::__private::checkpoint::RunCheckpoint,
            ) -> $crate::__private::anyhow::Result<::std::path::PathBuf> {
                $crate::__private::checkpoint::storage::save(self, checkpoint).await
            }
            pub async fn load_run_checkpoint(
                &self,
            ) -> $crate::__private::anyhow::Result<
                ::std::option::Option<$crate::__private::checkpoint::RunCheckpoint>,
            > {
                $crate::__private::checkpoint::storage::load(self).await
            }
            pub async fn clear_run_checkpoint(&mut self) -> $crate::__private::anyhow::Result<()> {
                $crate::__private::checkpoint::storage::clear(self).await
            }
        }
    };
}
