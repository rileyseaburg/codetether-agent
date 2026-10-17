//! create task push notification config RPC operation.
use super::*;
pub(super) async fn create_task_push_notification_config(
    service: &A2aServiceImpl,
    request: Request<proto::CreateTaskPushNotificationConfigRequest>,
) -> Result<Response<proto::TaskPushNotificationConfig>, Status> {
    let req = request.into_inner();
    let task_id = req.parent.strip_prefix("tasks/").unwrap_or(&req.parent);

    if service.store.get_task(task_id).is_none() {
        return Err(Status::not_found(format!("task {task_id} not found")));
    }

    let config = req
        .config
        .ok_or_else(|| Status::invalid_argument("missing config"))?;
    let pnc = config.push_notification_config.as_ref();

    let local_config = local::TaskPushNotificationConfig {
        id: task_id.to_string(),
        push_notification_config: local::PushNotificationConfig {
            url: pnc.map(|c| c.url.clone()).unwrap_or_default(),
            token: pnc.and_then(|c| {
                if c.token.is_empty() {
                    None
                } else {
                    Some(c.token.clone())
                }
            }),
            id: pnc.and_then(|c| {
                if c.id.is_empty() {
                    None
                } else {
                    Some(c.id.clone())
                }
            }),
        },
    };

    service
        .store
        .push_configs
        .entry(task_id.to_string())
        .or_default()
        .push(local_config);

    Ok(Response::new(config))
}
