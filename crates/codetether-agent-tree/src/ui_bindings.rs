//! Preserve typed TUI entry points for the shared agent hierarchy.
/// Declare the existing display-order function for an application agent entry.
/// # Usage
/// ```text
/// codetether_agent_tree::define_display_order!(SpawnedAgent);
/// ```
#[macro_export]
macro_rules! define_display_order {
    ($agent:ty) => {
        /// Full depth-first ordering of every spawned agent, with parents first.
        pub fn dfs_order(
            agents: &::std::collections::HashMap<::std::string::String, $agent>,
        ) -> ::std::vec::Vec<$crate::AgentNode> {
            $crate::dfs_order(agents)
        }
    };
}
/// Declare child-depth and spawn validation for an application agent entry.
/// # Usage
/// ```text
/// codetether_agent_tree::define_spawn_guards!(SpawnedAgent);
/// ```
#[macro_export]
macro_rules! define_spawn_guards {
    ($agent:ty) => {
        /// Depth a new child of `parent` would occupy (`None` parent → depth 0).
        pub fn child_depth(
            agents: &::std::collections::HashMap<::std::string::String, $agent>,
            parent: Option<&str>,
        ) -> Option<u8> {
            $crate::child_depth(agents, parent)
        }

        /// Validate a spawn under `parent`, returning the child depth or an error
        /// message (unknown parent or depth cap exceeded).
        pub fn validate_spawn(
            agents: &::std::collections::HashMap<::std::string::String, $agent>,
            parent: Option<&str>,
        ) -> Result<u8, ::std::string::String> {
            $crate::validate_spawn(agents, parent)
        }
    };
}
