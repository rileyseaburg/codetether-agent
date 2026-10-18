//! Construction of the application's collaboration capability group.

/// Declare collaboration catalog construction using existing tool/host types.
///
/// The application supplies the capability enum and legacy agent tool. Every
/// first-class collaboration tool uses the selected host; other capabilities
/// return `None` so their own catalog groups can handle them.
/// # Examples
/// ```text
/// codetether_collaboration_tools::declare_catalog!(ToolKind, AgentTool, AppHost);
/// ```
#[macro_export]
macro_rules! declare_catalog {
    ($kind:path, $agent:ty, $host:ty $(,)?) => {
        pub(crate) fn construct(
            kind: $kind,
        ) -> Option<::std::sync::Arc<dyn $crate::__private::Tool>> {
            use ::std::sync::Arc;
            use $crate::__private::Tool;
            use $kind as ToolKind;
            Some(match kind {
                ToolKind::Agent => Arc::new(<$agent>::new()) as Arc<dyn Tool>,
                ToolKind::SpawnAgent => {
                    Arc::new($crate::SpawnAgentTool::<$host>::new()) as Arc<dyn Tool>
                }
                ToolKind::FollowupTask => {
                    Arc::new($crate::FollowupTaskTool::<$host>::new()) as Arc<dyn Tool>
                }
                ToolKind::ListAgents => {
                    Arc::new($crate::ListAgentsTool::<$host>::new()) as Arc<dyn Tool>
                }
                ToolKind::WaitAgent => {
                    Arc::new($crate::WaitAgentTool::<$host>::new()) as Arc<dyn Tool>
                }
                ToolKind::InterruptAgent => {
                    Arc::new($crate::InterruptAgentTool::<$host>::new()) as Arc<dyn Tool>
                }
                ToolKind::CloseAgent => {
                    Arc::new($crate::CloseAgentTool::<$host>::new()) as Arc<dyn Tool>
                }
                ToolKind::ResumeAgent => {
                    Arc::new($crate::ResumeAgentTool::<$host>::new()) as Arc<dyn Tool>
                }
                ToolKind::SendInput => {
                    Arc::new($crate::SendInputTool::<$host>::new()) as Arc<dyn Tool>
                }
                ToolKind::SendMessage => {
                    Arc::new($crate::SendMessageTool::<$host>::new()) as Arc<dyn Tool>
                }
                _ => return None,
            })
        }
    };
}
