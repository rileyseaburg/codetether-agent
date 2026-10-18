//! Application registry sources and typed snapshot entry points.

/// Declare agent views using an application's existing session and store.
///
/// The visibility applies to the tool roster handler. Snapshot functions remain
/// public, while the application host is available within the embedding crate.
/// Registry lookup preserves the store's owner scoping and name resolution.
///
/// # Examples
/// ```text
/// codetether_agent_view::declare_application_view!(pub(crate); Session, store);
/// ```
#[macro_export]
macro_rules! declare_application_view {
    ($visibility:vis; $session:ty, $store:path $(,)?) => {
        pub use $crate::{AgentOrigin, AgentSnapshot};
        pub(crate) use host::ApplicationHost;

        mod host {
            use $store as store;
            pub type ApplicationHost = $crate::application::ApplicationHost<RegistrySource>;

            pub struct RegistrySource;
            impl $crate::application::Source for RegistrySource {
                type Session = $session;
                fn entries(parent: Option<&str>) -> Vec<store::AgentEntry> {
                    store::entries_for_parent(parent)
                }
                fn get_for_parent(name: &str, parent: &str) -> Option<store::AgentEntry> {
                    store::get_for_parent(name, Some(parent))
                }
                fn contains(name: &str) -> bool {
                    store::contains_name(name, None)
                }
            }
        }

        /// Return snapshots of all agents spawned through the agent tool.
        /// # Arguments
        /// This function takes no arguments.
        /// # Returns
        /// Current local and observed remote agent snapshots.
        /// # Examples
        /// ```text
        /// let snapshots = list_agent_tool_agents();
        /// ```
        pub fn list_agent_tool_agents() -> Vec<AgentSnapshot> {
            $crate::list::<ApplicationHost>(None)
        }

        /// Return agents owned by the given parent session.
        /// # Arguments
        /// * `parent_session_id` — Owner scope for the snapshots.
        /// # Returns
        /// Snapshots visible within that owner scope.
        /// # Examples
        /// ```text
        /// let snapshots = list_agent_tool_agents_for_parent("parent-id");
        /// ```
        pub fn list_agent_tool_agents_for_parent(parent_session_id: &str) -> Vec<AgentSnapshot> {
            $crate::list::<ApplicationHost>(Some(parent_session_id))
        }

        /// Find an agent only within the given parent session.
        /// # Arguments
        /// * `name` — Agent name to find.
        /// * `parent_session_id` — Required owner scope.
        /// # Returns
        /// The visible matching snapshot, or none.
        /// # Examples
        /// ```text
        /// let snapshot = find_agent_tool_agent_for_parent("reviewer", "parent-id");
        /// ```
        pub fn find_agent_tool_agent_for_parent(name: &str, parent_session_id: &str) -> Option<AgentSnapshot> {
            $crate::find::<ApplicationHost>(name, parent_session_id)
        }

        $visibility async fn handle_list(parent: Option<&str>) -> $crate::__private::Result<$crate::__private::ToolResult> {
            $crate::legacy_list::<ApplicationHost>(parent)
        }
    };
}
