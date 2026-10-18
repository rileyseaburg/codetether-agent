//! Resolve the configured gRPC listener address.
pub(super) fn resolve(hostname: &str) -> Result<std::net::SocketAddr, std::net::AddrParseError> {
    let grpc_port = std::env::var("CODETETHER_GRPC_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(50051);
    let grpc_addr: std::net::SocketAddr = format!("{}:{}", hostname, grpc_port).parse()?;
    Ok(grpc_addr)
}
