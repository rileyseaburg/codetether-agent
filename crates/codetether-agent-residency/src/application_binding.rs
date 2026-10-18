//! Application storage and prepared-session activation declarations.

/// Bind residency operations to existing application stores and overlays.
///
/// The two visibility arguments control agent-only operations and broader tool
/// access. Remaining arguments select the session, prepared record, registry,
/// persistence, trace cleanup and overlay function. Shared residency state
/// remains owned by the collaboration-state crate.
///
/// # Examples
/// ```text
/// codetether_agent_residency::declare_application_residency!(
///     pub(crate), pub(crate), Session, ClosedAgent, store, persistence,
///     clear_trace, apply_overlay,
/// );
/// ```
#[macro_export]
macro_rules! declare_application_residency {
    ($agent_vis:vis, $tool_vis:vis, $session:ty, $prepared:ty, $store:path, $persistence:path, $clear:path, $overlay:path $(,)?) => {
        mod host {
            use $store as store;
            use $persistence as persistence;

            pub struct ApplicationHost;
            pub struct Prepared($prepared);

            #[$crate::__private::async_trait]
            impl $crate::Storage for ApplicationHost {
                type Session = $session;
                fn get(id: &str) -> Option<store::AgentEntry> {
                    store::get(id)
                }
                fn resolve(target: &str, owner: Option<&str>) -> Option<(String, store::AgentEntry)> {
                    store::scope::resolve_for_parent(target, owner)
                }
                fn insert(entry: store::AgentEntry) {
                    store::insert(entry);
                }
                fn remove_runtime(id: &str) {
                    store::remove(id);
                    $clear(id);
                }
                async fn durable_id(owner: Option<&str>, target: &str) -> $crate::__private::Result<Option<String>> {
                    persistence::durable_id(owner, target).await
                }
                async fn close(name: &str, entry: &store::AgentEntry) -> $crate::__private::Result<()> {
                    persistence::close(name, entry).await
                }
            }

            #[$crate::__private::async_trait]
            impl $crate::Host for ApplicationHost {
                type Prepared = Prepared;
                async fn prepare_resume(owner: Option<&str>, target: &str) -> $crate::__private::Result<Prepared> {
                    persistence::prepare_resume(owner, target).await.map(Prepared)
                }
                fn entry_mut(prepared: &mut Prepared) -> &mut store::AgentEntry {
                    &mut prepared.0.entry
                }
                async fn activate_resume(prepared: Prepared) -> $crate::__private::Result<store::AgentEntry> {
                    persistence::activate_resume(prepared.0).await
                }
                async fn overlay(entry: &mut store::AgentEntry, config: &$crate::ResumeConfig) -> $crate::__private::Result<()> {
                    $overlay(entry, config).await
                }
            }
        }

        pub(crate) use $crate::EnsureOpen;
        pub(crate) use $crate::__private::residency::touch;
        $tool_vis use $crate::ResumeConfig;
        $agent_vis use $crate::__private::residency::acquire as acquire_transition;
        $agent_vis use $crate::__private::residency::stale::mark as mark_stale;

        $tool_vis async fn open(target: &str, owner: Option<&str>, config: ResumeConfig) -> $crate::__private::Result<EnsureOpen> {
            $crate::open::<host::ApplicationHost>(target, owner, config).await
        }

        $agent_vis async fn reserve(protected: Option<&str>) -> ::std::result::Result<$crate::__private::Slot, $crate::__private::ToolResult> {
            $crate::reserve::<host::ApplicationHost>(protected).await
        }

        $agent_vis fn forget(agent_id: &str) {
            $crate::__private::residency::forget(agent_id);
            $crate::__private::residency::stale::forget(agent_id);
        }
    };
}
