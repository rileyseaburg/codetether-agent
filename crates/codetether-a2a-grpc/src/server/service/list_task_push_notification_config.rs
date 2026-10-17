//! list task push notification config RPC operation.
use super::*;
pub(super) async fn list_task_push_notification_config(
    service: &A2aServiceImpl,
    request: Request<proto::ListTaskPushNotificationConfigRequest>,
) -> Result<Response<proto::ListTaskPushNotificationConfigResponse>, Status> {
    let req = request.into_inner();
    let task_id = req.parent.strip_prefix("tasks/").unwrap_or(&req.parent);

    let configs: Vec<proto::TaskPushNotificationConfig> = service
        .store
        .push_configs
        .get(task_id)
        .map(|cs| {
            cs.iter()
                .map(|c| proto::TaskPushNotificationConfig {
                    name: format!(
                        "tasks/{}/pushNotificationConfigs/{}",
                        task_id,
                        c.push_notification_config
                            .id
                            .as_deref()
                            .unwrap_or("default")
                    ),
                    push_notification_config: None,
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(Response::new(
        proto::ListTaskPushNotificationConfigResponse {
            configs,
            next_page_token: String::new(),
        },
    ))
}
