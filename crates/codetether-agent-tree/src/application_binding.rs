//! Application entry projection and tree query declarations.

/// Bind tree queries to an application's existing registry snapshots.
///
/// The entry type must provide `id()`, `name` and `owner_session_id`. The snapshot
/// callback accepts an optional owner and returns all entries when passed `None`.
/// All queries retain the shared tree's scoping and identity rules.
///
/// # Examples
/// ```text
/// codetether_agent_tree::declare_application_tree!(
///     pub(crate), AgentEntry, entries_for_parent,
/// );
/// ```
#[macro_export]
macro_rules! declare_application_tree {
    ($visibility:vis, $entry:ty, $snapshot:path $(,)?) => {
        $visibility use $crate::ListedAgent;

        mod host {
            pub struct TreeEntry($entry);
            impl $crate::Entry for TreeEntry {
                fn id(&self) -> &str {
                    self.0.id()
                }
                fn name(&self) -> &str {
                    &self.0.name
                }
                fn owner_session_id(&self) -> Option<&str> {
                    self.0.owner_session_id.as_deref()
                }
            }

            pub struct ApplicationHost;
            impl $crate::Host for ApplicationHost {
                type Entry = TreeEntry;
                fn entries() -> Vec<TreeEntry> {
                    $snapshot(None).into_iter().map(TreeEntry).collect()
                }
            }
        }

        $visibility fn list(current: &str, path_prefix: Option<&str>) -> $crate::__private::Result<Vec<ListedAgent>> {
            $crate::list::<host::ApplicationHost>(current, path_prefix)
        }

        $visibility fn resolve(current: &str, target: &str) -> $crate::__private::Result<Option<String>> {
            $crate::resolve::<host::ApplicationHost>(current, target)
        }

        $visibility fn canonical(current: &str, target: &str) -> $crate::__private::Result<Option<String>> {
            $crate::canonical::<host::ApplicationHost>(current, target)
        }
    };
}
