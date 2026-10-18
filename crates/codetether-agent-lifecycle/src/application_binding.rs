//! Application persistence, activation and trace bindings for close/resume.

/// Declare lifecycle entry points over an application's existing services.
///
/// The arguments select visibility, session type, persistence, residency, live
/// registry and trace cleanup. The shared lifecycle engine retains sequencing;
/// these operations delegate to the selected services without additional state.
///
/// # Examples
/// ```text
/// codetether_agent_lifecycle::declare_application_lifecycle!(
///     pub(super), Session, persistence, residency, store, clear_trace,
/// );
/// ```
#[macro_export]
macro_rules! declare_application_lifecycle {
    ($visibility:vis, $session:ty, $persistence:path, $residency:path, $store:path, $clear:path $(,)?) => {
        mod host {
            use $persistence as persistence;
            use $residency as residency;
            use $store as store;

            pub struct ApplicationHost;
            #[$crate::__private::async_trait]
            impl $crate::Host for ApplicationHost {
                type Session = $session;

                async fn durable_id(owner: Option<&str>, target: &str) -> $crate::__private::Result<Option<String>> {
                    persistence::durable_id(owner, target).await
                }

                async fn prepare_resume(
                    owner: Option<&str>,
                    target: &str,
                ) -> $crate::__private::Result<(String, store::AgentEntry)> {
                    let closed = persistence::prepare_resume(owner, target).await?;
                    Ok((closed.name, closed.entry))
                }

                async fn close(name: &str, entry: &store::AgentEntry) -> $crate::__private::Result<()> {
                    persistence::close(name, entry).await
                }

                async fn child_ids(parent: &str) -> $crate::__private::Result<Vec<String>> {
                    persistence::child_ids(parent).await
                }

                async fn open(target: &str, owner: Option<&str>, config: $crate::ResumeConfig) -> $crate::__private::Result<$crate::__private::EnsureOpen> {
                    residency::open(target, owner, config).await
                }

                fn get(id: &str) -> Option<store::AgentEntry> {
                    store::get(id)
                }

                fn remove_runtime(id: &str) {
                    store::remove(id);
                    $clear(id);
                }
            }
        }

        $visibility async fn close(
            target: &str,
            owner: Option<&str>,
        ) -> $crate::__private::Result<$crate::__private::ToolResult> {
            $crate::close::<host::ApplicationHost>(target, owner).await
        }

        $visibility async fn resume(
            target: &str,
            owner: Option<&str>,
            config: $crate::ResumeConfig,
        ) -> $crate::__private::Result<$crate::__private::ToolResult> {
            $crate::resume::<host::ApplicationHost>(target, owner, config).await
        }
    };
}
