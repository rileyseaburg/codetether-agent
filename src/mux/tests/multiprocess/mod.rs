//! Separate-process proof for workspace-bound mux servers.

mod child;
mod initial_shell;
mod kill_all;
mod process;
mod proof;
mod registry;
mod shared_workspace;
mod verify;

use crate::mux::client::MuxConnection;
use crate::mux::protocol::ClientRequest;

#[tokio::test]
async fn distinct_workspaces_run_in_separate_processes() {
    let root = tempfile::tempdir().unwrap().keep();
    let backend = root.join("backend");
    let frontend = root.join("frontend");
    let shared = root.join("shared");
    for workspace in [&backend, &frontend, &shared] {
        tokio::fs::create_dir_all(workspace).await.unwrap();
    }
    let mut alpha = process::start("alpha", &backend, &root).await;
    let mut beta = process::start("beta", &frontend, &root).await;
    assert_ne!(alpha.target.record.pid, beta.target.record.pid);

    let mut alpha_client = MuxConnection::connect(&alpha.target).await.unwrap();
    let alpha_state = verify::state(
        alpha_client
            .request(ClientRequest::CreateWindow { workspace: shared })
            .await
            .unwrap(),
    );
    let mut beta_client = MuxConnection::connect(&beta.target).await.unwrap();
    let beta_state = verify::state(beta_client.request(ClientRequest::Snapshot).await.unwrap());
    assert_eq!(alpha_state.session("alpha").unwrap().windows.len(), 2);
    let beta_session = beta_state.session("beta").unwrap();
    assert_eq!(beta_session.windows.len(), 1);
    assert_eq!(beta_session.windows[0].workspace, frontend);

    let artifact = proof::write(
        &root,
        &alpha.target.record,
        &beta.target.record,
        &alpha_state,
        &beta_state,
    )
    .await;
    verify::shutdown(&mut alpha_client, &mut alpha.child).await;
    verify::shutdown(&mut beta_client, &mut beta.child).await;
    println!("mux proof artifact: {}", artifact.display());
}
