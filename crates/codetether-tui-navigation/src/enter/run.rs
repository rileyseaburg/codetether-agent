use super::Host;
use codetether_tui_models::ViewMode;
/// Dispatch Enter to the handler for the active view.
/// # Arguments
/// * `host` - Native view state and Enter handlers.
/// # Returns
/// Completes after the selected handler; passive views do nothing.
/// # Examples
/// ```text
/// enter::dispatch(&mut native_host).await;
/// ```
pub async fn dispatch(host: &mut impl Host) {
    match host.view() {
        ViewMode::Sessions => host.sessions().await,
        ViewMode::FilePicker => host.file_picker(),
        ViewMode::Swarm => host.swarm().await,
        ViewMode::Ralph => host.ralph(),
        ViewMode::Bus if host.bus_filter_active() => host.bus_filter(),
        ViewMode::Bus => host.bus_detail(),
        ViewMode::Chat => host.chat().await,
        ViewMode::Model => host.model(),
        ViewMode::Settings => host.settings().await,
        ViewMode::Subagents => host.subagents(),
        ViewMode::Lsp
        | ViewMode::Rlm
        | ViewMode::Latency
        | ViewMode::Transport
        | ViewMode::Protocol
        | ViewMode::Inspector
        | ViewMode::Audit
        | ViewMode::Git
        | ViewMode::AuditLoop
        | ViewMode::Editor => {}
    }
}
