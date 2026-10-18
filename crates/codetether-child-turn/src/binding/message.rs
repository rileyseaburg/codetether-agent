//! Application message operations for child turns.
/// Supporting implementation for [crate::impl_child_turn].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_child_message {
    ($host:ty, $session:ty, $event:ty, $store:path, $remote:path, $begin:path) => {
        mod message {
            use $crate::__private::{MessageImage, Params, Result, ToolResult, async_trait};
            use $crate::Started;
            use $store as store;
            #[async_trait]
            impl $crate::MessageHost for $host {
                fn resolve(target: &str, owner: Option<&str>) -> Option<(String, String)> {
                    store::scope::resolve_for_parent(target, owner)
                        .map(|(id, entry)| (id, entry.name))
                }
                async fn remote(target: &str, text: &str, params: &Params) -> Result<ToolResult> {
                    $remote(target, text, params).await
                }
                async fn begin_message(
                    agent_id: &str,
                    text: String,
                    images: Vec<MessageImage>,
                    params: &Params,
                ) -> Result<Started<$event, $session>> {
                    $begin(agent_id, text, images, params).await
                }
            }
        }
    };
}
