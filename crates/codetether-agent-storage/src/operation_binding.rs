//! Typed application entry points for storage recovery and lifecycle operations.

/// Supporting declarations expanded by [`crate::declare_application_storage`].
#[doc(hidden)]
#[macro_export]
macro_rules! __declare_storage_operations {
    ($visibility:vis, $session:ty, $host:path) => {
        pub(crate) mod atomic {
            pub(crate) use $crate::atomic::{read, remove, write};
        }
        pub(crate) mod paths {
            pub(crate) use $crate::paths::{mailbox, manifest, root};
        }
        use $crate as manifest;
        use $crate::scan as manifest_scan;

        $visibility type ClosedAgent = $crate::ClosedAgent<$session>;
        $visibility use $crate::{child_ids, durable_id, exists, remove, save};

        $visibility async fn close(
            name: &str,
            entry: &$crate::__private::AgentEntry<$session>,
        ) -> $crate::__private::Result<()> {
            $crate::close(name, entry).await
        }

        $visibility async fn prepare_resume(
            owner: Option<&str>,
            target: &str,
        ) -> $crate::__private::Result<ClosedAgent> {
            $crate::prepare_resume::<$host>(owner, target).await
        }

        $visibility async fn activate_resume(
            closed: ClosedAgent,
        ) -> $crate::__private::Result<$crate::__private::AgentEntry<$session>> {
            $crate::activate_resume::<$host>(closed).await
        }

        pub(crate) async fn hydrate_parent(owner: Option<&str>) -> $crate::__private::Result<usize> {
            $crate::hydrate_parent::<$host>(owner).await
        }

        pub(crate) async fn reparent_owner(from: &str, to: &str) -> $crate::__private::Result<usize> {
            $crate::reparent_owner::<$host>(from, to).await
        }

        pub(crate) async fn hydrate_best_effort(parent: &str) {
            if let Err(error) = hydrate_parent(Some(parent)).await {
                $crate::__private::warn!(parent_session_id = parent, %error, "Agent recovery failed");
            }
        }

        #[cfg(test)]
        $visibility async fn is_open(owner: Option<&str>, target: &str) -> $crate::__private::Result<bool> {
            Ok(manifest_scan::find(owner, target)
                .await?
                .is_some_and(|manifest| manifest.lifecycle == manifest::Lifecycle::Open))
        }
    };
}
