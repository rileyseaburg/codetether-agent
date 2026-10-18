//! Application-local model-picker state adapters.

/// Declare the existing model-picker methods on a local AppState.
/// # Usage
/// Pass the state type and any native type/variant bindings required by the matcher.
#[macro_export]
macro_rules! __app_model_refresh {
    ($state:ty, $registry:ty, $model_mode:path) => {
        impl $state {
            pub fn start_model_refresh(&mut self, registry: ::std::sync::Arc<$registry>) {
                self.model_picker.start_model_refresh(registry);
            }
            pub fn drain_model_refresh(&mut self) {
                if let Some(status) = self
                    .model_picker
                    .drain_refresh_status(self.view_mode == $model_mode)
                {
                    self.status = status;
                }
            }

            /// Refresh the available models list from the provider registry.
            ///
            /// # Errors
            ///
            /// Returns an error if any provider call fails critically.
            pub async fn refresh_available_models(
                &mut self,
                registry: Option<&::std::sync::Arc<$registry>>,
            ) -> $crate::state_binding::anyhow::Result<$crate::state_binding::ModelRefreshSummary>
            {
                self.model_picker.refresh_available_models(registry).await
            }
        }
    };
}
