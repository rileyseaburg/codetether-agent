//! First-frame hydration before slow dependencies finish.
use super::{Fields, Session};
use std::path::Path;
/// Populate the initial frame and known peer/session identities.
/// # Arguments
/// Receives frontend fields, workspace, network permission, peer readiness and session.
/// # Returns
/// Updates the loading frame before slow startup work completes.
pub fn initial(
    host: &mut impl Fields,
    cwd: &Path,
    allow_network: bool,
    peer_ready: bool,
    session: &impl Session,
) {
    loading(host, cwd, allow_network);
    host.set_peer_ready(peer_ready);
    host.set_session_id(session.id().to_string());
}
/// Populate the loading-frame workspace, network and status fields.
/// # Arguments
/// Receives frontend fields, displayed workspace and requested network permission.
/// # Returns
/// Updates only the loading-frame fields.
pub fn loading(host: &mut impl Fields, cwd: &Path, allow_network: bool) {
    host.set_cwd(cwd.display().to_string());
    host.set_allow_network(allow_network);
    host.set_status("Loading providers and workspace...".to_string());
}
