//! Ordered preparation of a peer endpoint.
use super::*;

/// Shared setup: bind listener (port 0 → OS-assigned), auto-pick name if
/// missing, build card, register on bus, spawn explicit-seed discovery
/// loop, and start mDNS announce + browse if enabled.
///
/// Bind happens first so we know the effective port before publishing a
/// URL. Binding, local-address lookup and URL validation finish before any
/// bus registration or background tasks, so their errors leave no discovery
/// state behind.
///
/// # Arguments
///
/// Supply startup options, the shared bus, and application callbacks.
///
/// # Returns
///
/// The bound listener, router and running discovery tasks.
///
/// # Errors
///
/// Returns listener-binding, local-address or public-URL normalization errors.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_http::peer_startup::prepare_a2a;
/// let _prepare = prepare_a2a;
/// ```
pub async fn prepare_a2a(
    opts: SpawnOptions,
    bus: Arc<AgentBus>,
    hooks: StartupHooks,
) -> Result<A2APreparation> {
    let Binding {
        listener,
        local_addr,
        effective_port,
        bind_addr,
        agent_name,
        public_host,
        public_url,
    } = bind::bind(&opts).await?;

    // 4. Build card.
    let card = (hooks.build_card)(&agent_name, &public_url, opts.description.as_deref());

    bus.registry.register(card.clone());
    let bus_handle = bus.handle(&agent_name);
    let capabilities = card.skills.iter().map(|skill| skill.id.clone()).collect();
    bus_handle.announce_ready(capabilities);

    let discovery_task = seeds::start(&opts, &bus, &public_url, &agent_name, hooks);
    let (mdns_handle, mdns_intake_task, lan_tasks) = local::start(
        &opts,
        &bus,
        &public_host,
        local_addr,
        effective_port,
        &public_url,
        &agent_name,
        hooks,
    )
    .await;

    let router: Router = (hooks.build_router)(card, Arc::clone(&bus));

    Ok(A2APreparation {
        listener,
        router,
        discovery_task,
        mdns_intake_task,
        lan_tasks,
        mdns_handle,
        agent_name,
        bind_addr,
        public_url,
    })
}
