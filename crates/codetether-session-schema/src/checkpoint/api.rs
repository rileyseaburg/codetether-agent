//! Shared checkpoint api bindings for the local session schema.
/// Internal checkpoint implementation for the shared session declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_checkpoint_api {
    ($name:ident) => {
        impl $name {
            /// Capture complete canonical evidence from a consistent database snapshot.
            /// Pending messages are saved before scanning; raw history is streamed.
            /// # Errors
            /// Returns persistence, snapshot, integrity or decoding errors.
            pub async fn build_run_checkpoint(
                &self,
                objective: &str,
                max_steps: usize,
                workspace: &::std::path::Path,
            ) -> $crate::__private::anyhow::Result<$crate::__private::checkpoint::RunCheckpoint>
            {
                self.save().await?;
                let path = self.storage.path_for(&self.id).map(Ok).unwrap_or_else(|| {
                    $crate::__private::store::lifecycle::session_path(&self.id)
                })?;
                let (builder, _) = $crate::__private::store::fold_messages::<Self, _, _>(
                    &path,
                    &self.id,
                    $crate::__private::checkpoint::CheckpointBuilder::default(),
                    |builder, index, message| builder.push(index, &message),
                )
                .await?;
                Ok(builder.finish(
                    objective,
                    max_steps,
                    &self.id,
                    Some(workspace.to_path_buf()),
                ))
            }
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
