//! Provider registry loading for frontend relays.
/// Implementation detail of [crate::declare_frontend_autochat].
#[doc(hidden)]
#[macro_export]
macro_rules! __autochat_runtime {
    ($registry:path, $provider:path) => {
        use ::std::sync::Arc;
        use $crate::interactive::host::{Host, Registry};
        use $provider as Provider;
        use $registry as ProviderRegistry;
        pub(crate) struct Runtime;
        pub(crate) struct NativeRegistry(Arc<ProviderRegistry>);
        impl Registry for NativeRegistry {
            fn resolve_model(&self, model: &str) -> ::anyhow::Result<(Arc<dyn Provider>, String)> {
                self.0.resolve_model(model)
            }
        }
        impl Host for Runtime {
            type Registry = NativeRegistry;
            async fn load_registry() -> ::anyhow::Result<Self::Registry> {
                ProviderRegistry::shared_from_vault()
                    .await
                    .map(NativeRegistry)
            }
        }
    };
}
