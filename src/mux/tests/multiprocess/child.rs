//! Ignored helper executed as each proof daemon process.

#[tokio::test]
#[ignore]
async fn daemon() {
    let name = std::env::var("MUX_PROOF_NAME").unwrap();
    let workspace = std::env::var_os("MUX_PROOF_WORKSPACE").unwrap().into();
    crate::mux::server::serve(name, workspace, "127.0.0.1:0".parse().unwrap())
        .await
        .unwrap();
}
