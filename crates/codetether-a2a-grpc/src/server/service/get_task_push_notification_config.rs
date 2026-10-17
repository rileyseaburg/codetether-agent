//! get task push notification config RPC operation.
use super::*;
pub(super) async fn get_task_push_notification_config(
    service: &A2aServiceImpl,
    request: Request<proto::GetTaskPushNotificationConfigRequest>,
) -> Result<Response<proto::TaskPushNotificationConfig>, Status> {
    let req = request.into_inner();
    // name format: tasks/{task_id}/pushNotificationConfigs/{config_id}
    let parts: Vec<&str> = req.name.split('/').collect();
    if parts.len() < 4 {
        return Err(Status::invalid_argument("invalid name format"));
    }
    let task_id = parts[1];
    let config_id = parts[3];

    let configs = service
        .store
        .push_configs
        .get(task_id)
        .ok_or_else(|| Status::not_found("no configs for task"))?;

    let _found = configs
        .iter()
        .find(|c| c.push_notification_config.id.as_deref() == Some(config_id))
        .ok_or_else(|| Status::not_found("config not found"))?;

    Ok(Response::new(proto::TaskPushNotificationConfig {
        name: req.name,
        push_notification_config: None, // simplified
    }))
}
