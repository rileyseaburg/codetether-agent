//! Tonic method signatures and their operation handlers.
use super::*;
implement_service! {
    send_message: proto::SendMessageRequest => proto::SendMessageResponse,
    send_streaming_message: proto::SendMessageRequest => StreamResult<proto::StreamResponse>,
    get_task: proto::GetTaskRequest => proto::Task,
    cancel_task: proto::CancelTaskRequest => proto::Task,
    task_subscription: proto::TaskSubscriptionRequest => StreamResult<proto::StreamResponse>,
    create_task_push_notification_config: proto::CreateTaskPushNotificationConfigRequest => proto::TaskPushNotificationConfig,
    get_task_push_notification_config: proto::GetTaskPushNotificationConfigRequest => proto::TaskPushNotificationConfig,
    list_task_push_notification_config: proto::ListTaskPushNotificationConfigRequest => proto::ListTaskPushNotificationConfigResponse,
    delete_task_push_notification_config: proto::DeleteTaskPushNotificationConfigRequest => (),
    get_agent_card: proto::GetAgentCardRequest => proto::AgentCard,
}
