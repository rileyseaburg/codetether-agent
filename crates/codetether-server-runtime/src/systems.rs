//! Audit, Kubernetes, authentication and bus initialization in startup order.
use codetether_audit::{self as audit, AuditLog};
use codetether_bus_core::AgentBus;
use codetether_http_auth::AuthState;
use codetether_k8s::K8sManager;
use std::{sync::Arc, time::Instant};

pub(crate) async fn prepare(t0: Instant) -> (AuditLog, Arc<K8sManager>, AuthState, Arc<AgentBus>) {
    let audit_log = AuditLog::from_env();
    let _ = audit::init_audit_log(audit_log.clone());
    tracing::info!(elapsed_ms = t0.elapsed().as_millis(), "[startup] pre-k8s");
    let k8s = Arc::new(K8sManager::new().await);
    tracing::info!(elapsed_ms = t0.elapsed().as_millis(), "[startup] k8s done");
    if k8s.is_available() {
        tracing::info!("K8s self-deployment enabled");
    }
    let auth_state = AuthState::from_env();
    tracing::info!(
        token_len = auth_state.token().len(),
        "Auth is mandatory. Only /health is served without a Bearer token."
    );
    tracing::info!(
        audit_entries = audit_log.count().await,
        "Audit log initialized"
    );
    let bus = AgentBus::new().into_arc();
    codetether_bus_core::global::set_global(bus.clone());
    codetether_bus_s3::spawn_bus_s3_sink(bus.clone());
    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] bus created"
    );
    (audit_log, k8s, auth_state, bus)
}
