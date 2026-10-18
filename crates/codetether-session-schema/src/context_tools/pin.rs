//! pin tool storage adapter over the shared session schema.
/// Internal context tool binding for application-local session types.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_context_tool_pin {
    ($name:ident, $bus:ty, $resolver:path) => {
        #[$crate::__private::async_trait]
        impl $crate::__private::context_pin::PinSource for $name {
            fn page_count(&self) -> usize {
                self.pages.len()
            }
            fn set_page_kind(&mut self, index: usize, kind: $crate::__private::PageKind) {
                self.pages[index] = kind;
            }
            async fn persist(&mut self) -> $crate::__private::anyhow::Result<()> {
                $name::save(self).await
            }
        }
    };
}
