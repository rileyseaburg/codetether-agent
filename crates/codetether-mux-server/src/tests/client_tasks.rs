use super::ClientTasks;

#[tokio::test]
async fn completed_client_tasks_are_reaped_during_server_lifetime() {
    let mut clients = ClientTasks::new();
    for _ in 0..128 {
        clients.spawn(async {});
    }
    tokio::task::yield_now().await;

    assert_eq!(clients.len(), 128);
    while clients.len() > 0 {
        clients.reap().await;
    }

    assert_eq!(clients.len(), 0);
}
