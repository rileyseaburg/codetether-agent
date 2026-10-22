//! Application session operations for CLI runs, resumption and relays.
/// Implement native CLI session contracts using the existing session fields.
///
/// The import callback and default step limit remain application choices.
/// The invocation module must be able to access id, messages, metadata, bus,
/// max_steps and usage, and the session's normal creation/prompt/resume methods.
/// # Usage
/// ```text
/// codetether_native_cli::impl_run_session!(Session, import_session, DEFAULT_MAX_STEPS);
/// ```
#[macro_export]
macro_rules! impl_run_session {
    ($session:ty, $import:path, $max_steps:path $(,)?) => {
        #[$crate::__private::async_trait]
        impl $crate::__private::RunSession for $session {
            type Registry = $crate::__private::ProviderRegistry;
            fn messages(&self) -> &[$crate::__private::Message] {
                &self.messages
            }
            fn message_count(&self) -> usize {
                <$session>::message_count(self)
            }
            async fn build_checkpoint(
                &self,
                objective: &str,
                max_steps: usize,
                workspace: &::std::path::Path,
            ) -> $crate::__private::Result<$crate::__private::RunCheckpoint> {
                <$session>::build_run_checkpoint(self, objective, max_steps, workspace).await
            }
            fn set_max_steps(&mut self, max_steps: Option<usize>) {
                self.max_steps = max_steps;
            }
            fn default_max_steps() -> usize {
                $max_steps
            }
            async fn load_registry()
            -> $crate::__private::Result<::std::sync::Arc<$crate::__private::ProviderRegistry>>
            {
                $crate::__private::ProviderRegistry::shared_from_vault().await
            }
            async fn prompt(
                &mut self,
                message: &str,
            ) -> $crate::__private::Result<$crate::__private::SessionResult> {
                <$session>::prompt(self, message).await
            }
            async fn prompt_with_events(
                &mut self,
                message: &str,
                events: $crate::__private::mpsc::Sender<$crate::__private::SessionEvent<$session>>,
                registry: ::std::sync::Arc<$crate::__private::ProviderRegistry>,
            ) -> $crate::__private::Result<$crate::__private::SessionResult> {
                <$session>::prompt_with_events(self, message, events, registry).await
            }
        }
        #[$crate::__private::async_trait]
        impl $crate::__private::SessionSource for $session {
            fn id(&self) -> &str {
                &self.id
            }
            async fn create() -> $crate::__private::Result<Self> {
                <$session>::new().await
            }
            async fn import_codex(id: &str) -> $crate::__private::Result<Self> {
                $import(id).await
            }
            async fn resume(id: &str) -> $crate::__private::Result<Self> {
                <$session>::resume(id).await
            }
            async fn resume_last(
                workspace: Option<&::std::path::Path>,
            ) -> $crate::__private::Result<Self> {
                <$session>::resume_last(workspace).await
            }
        }
        impl $crate::__private::RuntimeSession for $session {
            fn set_knowledge_snapshot(&mut self, path: Option<::std::path::PathBuf>) {
                self.metadata.knowledge_snapshot = path;
            }
            fn attach_bus(&mut self, bus: ::std::sync::Arc<$crate::__private::AgentBus>) {
                self.bus = Some(bus);
            }
            fn usage(&self) -> &$crate::__private::Usage {
                &self.usage
            }
        }
        impl $crate::__private::SessionPolicy for $session {
            fn set_model(&mut self, model: String) {
                self.metadata.model = Some(model);
            }
            fn enable_auto_apply(&mut self) {
                self.metadata.auto_apply_edits = true;
            }
            fn set_max_steps(&mut self, max_steps: Option<usize>) {
                self.max_steps = max_steps;
            }
        }
        #[$crate::__private::async_trait]
        impl $crate::__private::RelaySession for $session {
            async fn new() -> $crate::__private::Result<Self> {
                <$session>::new().await
            }
            fn configure(
                &mut self,
                model: String,
                name: String,
                bus: ::std::sync::Arc<$crate::__private::AgentBus>,
                instructions: String,
            ) {
                self.metadata.model = Some(model);
                self.set_agent_name(name);
                self.bus = Some(bus);
                self.add_message($crate::__private::Message {
                    role: $crate::__private::Role::System,
                    content: vec![$crate::__private::ContentPart::Text { text: instructions }],
                });
            }
            async fn prompt(
                &mut self,
                prompt: &str,
            ) -> $crate::__private::Result<$crate::__private::SessionResult> {
                <$session>::prompt(self, prompt).await
            }
        }
    };
}
