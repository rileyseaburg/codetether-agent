//! Managed session runtime using existing application state.
/// Internal managed-session binding implementation.
#[doc(hidden)]
#[macro_export]
macro_rules! __managed_session_runtime {
    ($session:ty, $prior:path) => {
        #[$crate::session_binding::async_trait]
        impl $crate::ChildSession for $session {
            async fn create() -> $crate::session_binding::anyhow::Result<Self> {
                <$session>::new().await
            }
            fn set_agent_name(&mut self, name: ::std::string::String) {
                <$session>::set_agent_name(self, name);
            }
            fn set_title(&mut self, title: ::std::string::String) {
                <$session>::set_title(self, title);
            }
            fn has_bus(&self) -> bool {
                self.bus.is_some()
            }
            fn attach_bus(&mut self) {
                self.bus = $crate::session_binding::global();
            }
            async fn save_updated(&mut self) -> $crate::session_binding::anyhow::Result<()> {
                self.updated_at = $crate::session_binding::Utc::now();
                self.save().await
            }
            async fn load_parent_policy(
                parent_id: &str,
            ) -> $crate::session_binding::anyhow::Result<bool> {
                let load = <$session>::load_tail(parent_id, 16).await?;
                Ok($prior(&load.session))
            }
        }
    };
}
