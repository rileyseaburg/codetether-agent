//! Separate-process proof for concurrent named mux sessions.

mod child;
mod kill_all;
mod process;
mod proof;
mod verify;

use crate::mux::client::MuxConnection;
use crate::mux::protocol::ClientRequest;

#[tokio::test]
async fn multiple_named_servers_run_in_separate_processes() {
    let root = tempfile::tempdir().unwrap().keep();
    let backend = root.join("backend");
    let frontend = root.join("frontend");
    let shared = root.join("shared");
    for workspace in [&backend, &frontend, &shared] {
        tokio::fs::create_dir_all(workspace).await.unwrap();
    }
    let mut alpha = process::start("alpha", &backend, &root).await;
    let mut beta = process::start("beta", &frontend, &root).await;
    assert_ne!(alpha.record.pid, beta.record.pid);

    let mut alpha_client = MuxConnection::connect(&alpha.record).await.unwrap();
    let alpha_state = verify::state(
        alpha_client
            .request(ClientRequest::CreateWindow { workspace: shared })
            .await
            .unwrap(),
    );
    let mut beta_client = MuxConnection::connect(&beta.record).await.unwrap();
    let beta_state = verify::state(beta_client.request(ClientRequest::Snapshot).await.unwrap());
    assert_eq!(alpha_state.windows.len(), 2);
    assert_eq!(beta_state.windows.len(), 1);
    assert_eq!(beta_state.windows[0].workspace, frontend);

    let artifact = proof::write(
        &root,
        &alpha.record,
        &beta.record,
        &alpha_state,
        &beta_state,
    )
    .await;
    verify::shutdown(&mut alpha_client, &mut alpha.child).await;
    verify::shutdown(&mut beta_client, &mut beta.child).await;
    println!("mux proof artifact: {}", artifact.display());
}
