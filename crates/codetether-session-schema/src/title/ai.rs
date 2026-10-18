//! Shared session title ai operations.
/// Internal title binding for an application-local session.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_title_ai {
    ($name:ident, $registry:ty) => {
        impl $name {
            pub(crate) async fn generate_ai_title_with(
                &mut self,
                provider: &dyn $crate::__private::Provider,
                model: &str,
            ) -> $crate::__private::anyhow::Result<()> {
                $crate::__private::titles::generate_with(self, provider, model).await
            }
        }
    };
}
