//! delete task push notification config RPC operation.
use super::*;
pub(super) async fn delete_task_push_notification_config(
    service: &A2aServiceImpl,
    request: Request<proto::DeleteTaskPushNotificationConfigRequest>,
) -> Result<Response<()>, Status> {
    let req = request.into_inner();
    let parts: Vec<&str> = req.name.split('/').collect();
    if parts.len() < 4 {
        return Err(Status::invalid_argument("invalid name format"));
    }
    let task_id = parts[1];
    let config_id = parts[3];

    if let Some(mut configs) = service.store.push_configs.get_mut(task_id) {
        configs.retain(|c| c.push_notification_config.id.as_deref() != Some(config_id));
    }

    Ok(Response::new(()))
}
