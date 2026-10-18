//! Shared session title selection operations.
/// Internal title binding for an application-local session.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_title_selection {
    ($name:ident, $registry:ty) => {
        impl $name {
            /// Generate an AI title with the preferred configured provider.
            ///
            /// Falls back to the first-message heuristic when no provider is usable.
            ///
            /// # Errors
            ///
            /// Returns an error if heuristic title generation fails.
            pub async fn generate_ai_title(
                &mut self,
                registry: &$registry,
            ) -> $crate::__private::anyhow::Result<()> {
                let providers = registry.list();
                let Some(name) =
                    $crate::__private::provider_policy::choose_default_provider(&providers)
                else {
                    return self.generate_title().await;
                };
                let Some(provider) = registry.get(name) else {
                    return self.generate_title().await;
                };
                let model = $crate::__private::provider_policy::default_model_for_provider(name);
                self.generate_ai_title_with(provider.as_ref(), &model).await
            }
        }
    };
}
