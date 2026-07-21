//! Tab focus tests for parent-scoped mDNS agents.

use super::*;

#[test]
fn tab_selects_the_observed_mdns_agent() {
    let suffix = uuid::Uuid::new_v4();
    let name = format!("peer-{suffix}");
    let parent = format!("parent-{suffix}");
    crate::tool::agent::bridge::record_remote_turn(&name, &parent, "inspect", "done");
    let mut app = App::default();
    app.state.session_id = Some(parent);
    app.state.set_view_mode(ViewMode::Chat);

    handle_tab(&mut app);

    assert_eq!(
        app.state.active_spawned_agent.as_deref(),
        Some(name.as_str())
    );
}
