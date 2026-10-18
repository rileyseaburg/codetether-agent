//! Repeated proof of zero-config collaboration across independent processes.

#[path = "a2a_mdns_process/mod.rs"]
mod support;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn three_independent_process_pairs_collaborate_without_seeds() {
    unsafe { std::env::set_var("CODETETHER_AUTH_TOKEN", "observer-control") };
    let proof = support::artifact::ProofDirectory::new();
    for round in 0..3 {
        support::round::run(round, proof.path()).await;
    }
}
