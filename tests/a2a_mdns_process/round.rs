use super::{agent, auth, discovery, process::PeerProcess};
use codetether_agent::a2a::spawn::{SpawnOptions, start_a2a_in_background};
use codetether_agent::bus::AgentBus;
use std::path::Path;

pub async fn run(round: usize, proof_directory: &Path) {
    let suffix = &uuid::Uuid::new_v4().simple().to_string()[..8];
    let alpha_name = format!("proof-{round}-alpha-{suffix}");
    let beta_name = format!("proof-{round}-beta-{suffix}");
    let observer_name = format!("proof-{round}-observer-{suffix}");
    let mut options = SpawnOptions::auto();
    options.name = Some(observer_name);
    options.auto_introduce = false;
    let observer = start_a2a_in_background(options, AgentBus::new().into_arc())
        .await
        .expect("start first-party observer");
    let alpha_control = format!("alpha-control-{suffix}");
    let beta_control = format!("beta-control-{suffix}");
    let mut alpha = PeerProcess::start(proof_directory, &alpha_name, &alpha_control);
    let mut beta = PeerProcess::start(proof_directory, &beta_name, &beta_control);
    let process_ids = (alpha.id(), beta.id());
    let routes = discovery::reciprocal(&mut alpha, &mut beta, &alpha_name, &beta_name).await;
    assert!(alpha.log().contains("bind_addr=0.0.0.0:"));
    assert!(beta.log().contains("bind_addr=0.0.0.0:"));
    agent::assert_remote_turn(&beta_name).await;
    auth::assert_boundary(&routes.beta, &alpha_control, &beta_control).await;
    drop((alpha, beta, observer));
    let manifest = format!(
        "alpha={alpha_name} pid={}\nbeta={beta_name} pid={}\nendpoint={}\n\
         reciprocal_mdns=true\nreciprocal_intro=true\nagent_tool_turn=true\n\
         workspace_context=true\nraw_endpoint_hidden=true\n\
         missing_token_401=true\nforeign_control_401=true\nown_control_200=true\n",
        process_ids.0, process_ids.1, routes.beta
    );
    std::fs::write(
        proof_directory.join(format!("round-{round}.passed")),
        manifest,
    )
    .expect("write proof manifest");
}
