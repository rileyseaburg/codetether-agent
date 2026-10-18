//! Background A2A and voice gRPC-Web listener.
use codetether_a2a_core::AgentCard;
use codetether_bus_core::AgentBus;
use std::sync::Arc;
use tonic_web::GrpcWebLayer;
/// Start the A2A and voice services on the configured gRPC port.
/// # Arguments
/// * `hostname` - Listener hostname used with the configured/default port.
/// * `agent_card` - Existing agent description passed to task storage.
/// * `bus` - Shared live event bus.
/// # Returns
/// Unit after spawning the detached listener task. Listener failures are logged.
/// # Errors
/// Returns an error when the configured address cannot be parsed.
/// # Examples
/// ```no_run
/// # fn example(card: codetether_a2a_core::AgentCard, bus: std::sync::Arc<codetether_bus_core::AgentBus>) -> Result<(), std::net::AddrParseError> {
/// codetether_a2a_grpc::listener::spawn("127.0.0.1", card, bus)?;
/// # Ok(()) }
/// ```
pub fn spawn(
    hostname: &str,
    agent_card: AgentCard,
    bus: Arc<AgentBus>,
) -> Result<(), std::net::AddrParseError> {
    let grpc_addr = super::address::resolve(hostname)?;
    let grpc_store = crate::server::GrpcTaskStore::with_bus(agent_card, bus.clone());
    let grpc_service = grpc_store.into_service();
    let voice_service = crate::voice::VoiceServiceImpl::new(bus.clone()).into_service();
    tokio::spawn(async move {
        tracing::info!(addr = %grpc_addr, "Starting gRPC A2A server");

        // Configure CORS for marketing site (production + local dev)
        let cors = codetether_server_cors::grpc();

        if let Err(error) = tonic::transport::Server::builder()
            .accept_http1(true)
            .layer(cors)
            .layer(GrpcWebLayer::new())
            .add_service(grpc_service)
            .add_service(voice_service)
            .serve(grpc_addr)
            .await
        {
            tracing::warn!(addr = %grpc_addr, error = ?error, "gRPC A2A server exited");
        }
    });
    Ok(())
}
