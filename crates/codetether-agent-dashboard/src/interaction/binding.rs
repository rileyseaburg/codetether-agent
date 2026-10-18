//! Frontend selected-agent state and dashboard navigation effects.
/// Declare dashboard Enter handling with parent-scoped tool-agent lookup.
///
/// The shared interaction policy keeps its local/tool/swarm precedence and
/// stale-selection handling. Frontend callbacks retain focus and registry access.
/// # Examples
/// See the frontend subagent Enter module for concrete callback selection.
#[macro_export]
macro_rules! declare_frontend_dashboard_input {
    ($app:path, $mode:path, $swarm:path; $lookup:path, $cycle:path $(,)?) => {
        use $app as App;
        mod host {
            use super::App;
            use $mode as ViewMode;
            use $swarm as SwarmViewState;
            impl $crate::interaction::Host for App {
                fn selected(&self) -> Option<&str> {
                    self.state.active_spawned_agent.as_deref()
                }
                fn clear_selected(&mut self) {
                    self.state.active_spawned_agent = None;
                }
                fn local_contains(&self, name: &str) -> bool {
                    self.state.spawned_agents.contains_key(name)
                }
                fn owned_tool_contains(&self, name: &str) -> bool {
                    self.state
                        .session_id
                        .as_deref()
                        .is_some_and(|parent| $lookup(name, parent).is_some())
                }
                fn swarm(&self) -> &SwarmViewState {
                    &self.state.swarm
                }
                fn swarm_mut(&mut self) -> &mut SwarmViewState {
                    &mut self.state.swarm
                }
                fn show_swarm(&mut self) {
                    self.state.set_view_mode(ViewMode::Swarm);
                }
                fn open_local_detail(&mut self) {
                    self.state.subagent_detail_mode = true;
                    self.state.subagent_detail_scroll = 0;
                }
                fn cycle_focus(&mut self) {
                    $cycle(self);
                }
                fn set_status(&mut self, status: String) {
                    self.state.status = status;
                }
            }
        }
        /// Handle Enter on the subagent dashboard using shared selection policy.
        pub(super) fn dispatch(app: &mut App) {
            $crate::interaction::enter(app);
        }
    };
}
