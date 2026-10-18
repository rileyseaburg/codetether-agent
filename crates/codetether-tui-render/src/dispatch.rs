//! Exhaustive active-view dispatch.
use crate::{Host, chat};
use codetether_tui_models::ViewMode;
use ratatui::Frame;
pub(super) fn render(frame: &mut Frame, host: &mut impl Host) {
    match host.view() {
        ViewMode::Chat => chat::render(frame, host),
        ViewMode::Sessions => host.sessions(frame),
        ViewMode::Swarm => host.swarm(frame),
        ViewMode::Ralph => host.ralph(frame),
        ViewMode::Bus => host.bus(frame),
        ViewMode::Subagents => host.subagents(frame),
        ViewMode::Model => host.model(frame),
        ViewMode::Settings => host.settings(frame),
        ViewMode::Lsp => host.lsp(frame),
        ViewMode::Rlm => host.rlm(frame),
        ViewMode::Latency => host.latency(frame),
        ViewMode::Transport => host.transport(frame),
        ViewMode::Protocol => host.protocol(frame),
        ViewMode::FilePicker => host.file_picker(frame),
        ViewMode::Inspector => host.inspector(frame),
        ViewMode::Audit => host.audit(frame),
        ViewMode::Git => host.git(frame),
        ViewMode::AuditLoop => host.audit_loop(frame),
        ViewMode::Editor => host.editor(frame),
    }
}
