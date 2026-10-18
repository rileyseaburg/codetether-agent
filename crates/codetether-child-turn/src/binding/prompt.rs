//! Application prompt operations for child turns.
/// Supporting implementation for [crate::impl_child_turn].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_child_prompt {
    ($host:ty, $session:ty, $event:ty, $registry:ty, $get:path) => {
        mod prompt {
            use ::std::sync::Arc;
            use $crate::__private::{MessageImage, Result, async_trait, mpsc};
            #[async_trait]
            impl $crate::PromptHost for $host {
                type Registry = $registry;
                async fn registry() -> Result<Arc<$registry>> {
                    $get().await
                }
                async fn prompt(
                    mut session: $session,
                    text: String,
                    images: Option<Vec<MessageImage>>,
                    tx: mpsc::Sender<$event>,
                    registry: Arc<$registry>,
                ) -> Result<$session> {
                    match images {
                        Some(images) => session
                            .prompt_with_events_and_images(
                                &text,
                                images.into_iter().map(Into::into).collect(),
                                tx,
                                registry,
                            )
                            .await
                            .map(|_| ()),
                        None => session
                            .prompt_with_events(&text, tx, registry)
                            .await
                            .map(|_| ()),
                    }
                    .map(|_| session)
                }
            }
        }
    };
}
