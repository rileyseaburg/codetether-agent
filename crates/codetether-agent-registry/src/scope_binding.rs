//! Owner-scoped lookup and lineage declarations for the application registry.

/// Supporting scope declarations for [`crate::declare_application_registry`].
#[doc(hidden)]
#[macro_export]
macro_rules! __declare_registry_scope {
    ($visibility:vis; $registry:ident, $resolve:path) => {
        $visibility mod scope {
            use super::{$registry, AgentEntry};

            /// Resolve a tree path, ID or unique name within an owner scope.
            pub fn resolve_for_parent(
                target: &str,
                parent: Option<&str>,
            ) -> Option<(String, AgentEntry)> {
                $registry.resolve_for_parent(target, parent, |parent, target| {
                    $resolve(parent, target).ok().flatten()
                })
            }

            /// Snapshot entries belonging to the requested owner.
            pub fn entries_for_parent(parent: Option<&str>) -> Vec<AgentEntry> {
                $registry.entries_for_parent(parent)
            }

            /// Return the entry selected by scoped identity resolution.
            pub fn get_for_parent(target: &str, parent: Option<&str>) -> Option<AgentEntry> {
                resolve_for_parent(target, parent).map(|(_, entry)| entry)
            }

            /// Find the parent display name and depth for a new child.
            pub fn lineage_for_session(session_id: Option<&str>) -> (Option<String>, u8) {
                $registry.lineage_for_session(session_id)
            }
        }

        $visibility use scope::{entries_for_parent, get_for_parent, lineage_for_session};
    };
}
