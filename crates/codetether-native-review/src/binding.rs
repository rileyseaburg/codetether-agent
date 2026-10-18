//! Application-facing reviewer declarations with explicit catalog and loop selection.

/// Declare the direct reviewer, read-only catalog and background dispatcher.
///
/// The two type arguments select the application's existing tool registry and
/// agent loop. Both review entry points share the native runtime, providers and
/// goal lookup. Invoke once in the application's review module.
///
/// # Examples
/// ```text
/// codetether_native_review::declare_review!(Tools, AgentLoop);
/// ```
#[macro_export]
macro_rules! declare_review {
    ($tools:ty, $agent:ty $(,)?) => {
        pub use $crate::{ReviewOutcome, ReviewSubject, ReviewVerdict, parse, prompt, runtime};

        mod host {
            pub(super) type Native = $crate::Reviewer<$tools, $agent>;
        }

        /// Review a proposed change with the configured read-only reviewer.
        ///
        /// # Arguments
        /// Receives review settings, installed providers, an optional fallback
        /// model, the workspace and the proposed change.
        /// # Returns
        /// The review verdict, including escalation when review cannot run.
        /// # Examples
        /// ```text
        /// let verdict = review(&config, &registry, None, workspace, subject).await;
        /// ```
        pub async fn review(
            config: &$crate::__private::ReviewConfig,
            registry: &$crate::__private::ProviderRegistry,
            session_model: Option<&str>,
            workspace: ::std::path::PathBuf,
            subject: $crate::ReviewSubject,
        ) -> $crate::ReviewVerdict {
            $crate::__private::review::<host::Native>(
                config,
                registry,
                session_model,
                workspace,
                subject,
            )
            .await
        }

        /// Build the default catalog with every shared-policy mutator removed.
        ///
        /// # Arguments
        /// The tool catalog is selected by the enclosing declaration.
        /// # Returns
        /// The restricted tool registry used by the reviewer.
        /// # Examples
        /// ```text
        /// assert!(read_only_tools().get("write").is_none());
        /// ```
        pub fn read_only_tools() -> $crate::__private::ToolRegistry<$tools> {
            $crate::__private::application::read_only_tools::<$tools>()
        }

        pub(crate) mod dispatch {
            pub(crate) use $crate::__private::dispatch::Context;

            pub(crate) fn start(
                item: &$crate::__private::ApprovalSnapshot,
                context: impl FnOnce() -> Context,
            ) {
                $crate::__private::dispatch::start::<super::host::Native>(item, context);
            }
        }
    };
}
