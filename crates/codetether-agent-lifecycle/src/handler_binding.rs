//! Typed application handlers for shared interruption and removal workflows.

/// Declare an interrupt handler using the selected application host.
///
/// # Examples
/// ```text
/// codetether_agent_lifecycle::declare_interrupt_handler!(pub(crate) fn handle; Native);
/// ```
#[macro_export]
macro_rules! declare_interrupt_handler {
    ($visibility:vis fn $name:ident; $(,)? $host:ty) => {
        /// Interrupt a scoped child, optionally persisting the developer marker.
        $visibility async fn $name(
            target: &str,
            parent: Option<&str>,
            marker_enabled: bool,
        ) -> $crate::__private::Result<$crate::__private::ToolResult> {
            $crate::interrupt::<$host>(target, parent, marker_enabled).await
        }
    };
}

/// Declare a permanent child-removal handler using the selected host.
///
/// # Examples
/// ```text
/// codetether_agent_lifecycle::declare_kill_handler!(pub(crate) fn handle; Native);
/// ```
#[macro_export]
macro_rules! declare_kill_handler {
    ($visibility:vis fn $name:ident; $(,)? $host:ty) => {
        /// Remove a scoped child through the shared lifecycle workflow.
        $visibility async fn $name(
            target: &str,
            parent: Option<&str>,
        ) -> $crate::__private::Result<$crate::__private::ToolResult> {
            $crate::kill::<$host>(target, parent).await
        }
    };
}
