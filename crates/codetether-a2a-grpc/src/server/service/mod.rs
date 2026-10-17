//! RPC dispatch into focused operation handlers.
use super::*;
mod cancel_task;
mod create_task_push_notification_config;
mod delete_task_push_notification_config;
mod get_agent_card;
mod get_task;
mod get_task_push_notification_config;
mod list_task_push_notification_config;
mod send_message;
mod send_streaming_message;
mod send_streaming_message_stream;
mod task_subscription;
mod task_subscription_stream;
macro_rules! implement_service {
    ($($method:ident: $request:ty => $response:ty,)*) => {
        #[tonic::async_trait]
        impl A2aService for A2aServiceImpl {
            type SendStreamingMessageStream = StreamResult<proto::StreamResponse>;
            type TaskSubscriptionStream = StreamResult<proto::StreamResponse>;
            $(async fn $method(&self, request: Request<$request>) -> Result<Response<$response>, Status> {
                $method::$method(self, request).await
            })*
        }
    };
}
mod adapter;
