//! budget tool storage adapter over the shared session schema.
/// Internal context tool binding for application-local session types.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_context_tool_budget {
    ($name:ident, $bus:ty, $resolver:path) => {
        impl $crate::__private::context_budget::BudgetSource for $name {
            const WINDOW: usize = $crate::__private::store::window::WINDOW;
            fn messages_len(&self) -> usize {
                self.messages.len()
            }
            fn cached_summaries(&self) -> usize {
                self.summary_index.len()
            }
            fn pages(&self) -> &[$crate::__private::PageKind] {
                self.pages.as_slice()
            }
        }
    };
}
