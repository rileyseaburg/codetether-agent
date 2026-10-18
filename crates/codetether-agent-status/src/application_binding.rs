//! Application status reporting and lifecycle publication declarations.

/// Declare status and publication modules over the existing live registry.
///
/// The module names and registry path are supplied by the application. Both
/// reporting and publication project the same live sessions; task ownership
/// mapping continues to use the shared session-based task IDs.
///
/// # Examples
/// ```text
/// codetether_agent_status::declare_application_status!(
///     bus_publish, status; crate::live_store
/// );
/// ```
#[macro_export]
macro_rules! declare_application_status {
    ($publication:ident, $status:ident; $store:path) => {
        mod $publication {
            mod host {
                use $crate::{PublishHost, PublishedAgent};
                use $store as store;

                pub struct ApplicationHost;
                impl PublishHost for ApplicationHost {
                    fn get(agent_id: &str) -> Option<PublishedAgent> {
                        store::get(agent_id).map(|entry| PublishedAgent {
                            id: entry.session.id,
                            name: entry.name,
                            owner_session_id: entry.owner_session_id,
                        })
                    }
                }
            }

            pub(super) mod lifecycle {
                pub(crate) use $crate::terminal_summary;
                pub(crate) fn task_to_agent(
                    parent: Option<&str>,
                ) -> ::std::collections::HashMap<String, String> {
                    $crate::task_to_agent::<super::super::$status::ApplicationHost>(parent)
                }
            }

            pub(super) fn announce_working(agent_id: &str, summary: impl Into<String>) {
                $crate::publish::announce_working::<host::ApplicationHost>(agent_id, summary);
            }
            pub(super) fn announce_done(agent_id: &str, success: bool, summary: impl Into<String>) {
                $crate::publish::announce_done::<host::ApplicationHost>(agent_id, success, summary);
            }
            pub(super) fn announce_result(agent_id: &str, response: &str, error: Option<&str>) {
                $crate::publish::announce_result::<host::ApplicationHost>(
                    agent_id, response, error,
                );
            }
        }

        mod $status {
            use super::$publication::lifecycle;
            use $crate::{AgentView, Host};
            use $store as store;

            pub(super) struct ApplicationHost;
            impl Host for ApplicationHost {
                fn agents(parent: Option<&str>) -> Vec<AgentView> {
                    store::entries_for_parent(parent)
                        .into_iter()
                        .map(|entry| AgentView {
                            id: entry.id().to_string(),
                            messages: entry.session.messages.len(),
                            name: entry.name,
                        })
                        .collect()
                }
                fn task_to_agent(
                    parent: Option<&str>,
                ) -> ::std::collections::HashMap<String, String> {
                    lifecycle::task_to_agent(parent)
                }
            }

            pub(super) fn handle_status(parent: Option<&str>) -> $crate::__private::ToolResult {
                $crate::report::<ApplicationHost>(parent)
            }
        }
    };
}
