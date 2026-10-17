//! Bind a listener before publishing identity or spawning discovery.
use super::*;
pub(super) struct Binding {
    pub listener: tokio::net::TcpListener,
    pub local_addr: SocketAddr,
    pub effective_port: u16,
    pub bind_addr: String,
    pub agent_name: String,
    pub public_host: String,
    pub public_url: String,
}
pub(super) async fn bind(opts: &SpawnOptions) -> Result<Binding> {
    // 1. Bind first so we know the effective port (handles --port 0) AND so
    //    a bind failure short-circuits cleanly without leaving any
    //    background task or stale bus registry entry behind.
    let requested_bind_addr = format!("{}:{}", opts.hostname, opts.port);
    let listener = tokio::net::TcpListener::bind(&requested_bind_addr)
        .await
        .with_context(|| format!("Failed to bind A2A peer on {requested_bind_addr}"))?;
    let local_addr = listener
        .local_addr()
        .context("Failed to read local addr from listener")?;
    let effective_port = local_addr.port();
    let bind_addr = format!("{}:{}", opts.hostname, effective_port);

    // 2. Auto-pick name if not provided.
    let agent_name = opts.name.clone().unwrap_or_else(auto_agent_name);

    // 3. Derive public_url from effective bind addr if not supplied.
    //    A wildcard host (`0.0.0.0` / `::`) is not a usable URL — pick the
    //    first non-loopback IPv4 address on a real interface so the card
    //    advertises something callers can actually reach.
    let public_host = match opts.hostname.as_str() {
        "0.0.0.0" | "::" | "[::]" => first_lan_ipv4().unwrap_or_else(|| "127.0.0.1".to_string()),
        other => other.to_string(),
    };
    let public_url = opts
        .public_url
        .clone()
        .unwrap_or_else(|| format!("http://{public_host}:{effective_port}"));
    let public_url = normalize_base_url(&public_url)?;

    Ok(Binding {
        listener,
        local_addr,
        effective_port,
        bind_addr,
        agent_name,
        public_host,
        public_url,
    })
}
