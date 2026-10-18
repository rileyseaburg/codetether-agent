//! Frontend prompt execution and successful directory restoration.
/// Declare prompt execution inside the selected question scope.
///
/// The original directory is restored after successful execution; errors retain
/// the existing early-return behavior for the surrounding runtime to handle.
/// # Examples
/// See the frontend worktree-result module for concrete runtime types.
#[macro_export]
macro_rules! declare_frontend_prompt_run {
    ($session:path, $image:path, $event:path, $registry:path; $scope:path $(,)?) => {
        use ::std::{path::PathBuf, sync::Arc};
        use ::tokio::sync::mpsc;
        use $event as SessionEvent;
        use $image as ImageAttachment;
        use $registry as ProviderRegistry;
        use $session as Session;
        /// Run the prompt against the provider and restore the
        /// original directory on completion.
        pub(crate) async fn run_prompt(
            session: &mut Session,
            prompt: &str,
            images: Vec<ImageAttachment>,
            event_tx: mpsc::Sender<SessionEvent>,
            registry: Arc<ProviderRegistry>,
            original_dir: Option<PathBuf>,
        ) -> anyhow::Result<()> {
            $scope(
                session.id.clone(),
                session.prompt_with_events_and_images(prompt, images, event_tx, registry),
            )
            .await?;
            session.metadata.directory = original_dir;
            Ok(())
        }
    };
}
