//! Shared checkpoint host bindings for the local session schema.
/// Internal checkpoint implementation for the shared session declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_checkpoint_host {
    ($name:ident) => {
        #[$crate::__private::async_trait]
        impl $crate::__private::checkpoint::storage::Storage for $name {
            fn id(&self) -> &str {
                &self.id
            }
            fn checkpoint(
                &self,
            ) -> ::std::option::Option<&$crate::__private::checkpoint::RunCheckpoint> {
                self.metadata.run_checkpoint.as_ref()
            }
            fn set_checkpoint(
                &mut self,
                checkpoint: ::std::option::Option<$crate::__private::checkpoint::RunCheckpoint>,
            ) {
                self.metadata.run_checkpoint = checkpoint;
            }
            fn checkpoint_cleared(&self) -> bool {
                self.metadata.run_checkpoint_cleared
            }
            fn set_checkpoint_cleared(&mut self, cleared: bool) {
                self.metadata.run_checkpoint_cleared = cleared;
            }
            fn sessions_dir() -> $crate::__private::anyhow::Result<::std::path::PathBuf> {
                $name::sessions_dir()
            }
            async fn save_session(&self) -> $crate::__private::anyhow::Result<()> {
                $name::save(self).await
            }
        }
    };
}
