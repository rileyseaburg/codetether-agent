//! Restore implementation for the shared session schema.
/// Internal storage binding for application-local session types.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_storage_restore {
    ($name:ident) => {
        impl $crate::__private::store::Restore for $name {
            fn decode_header(header: &str) -> $crate::__private::anyhow::Result<Self> {
                $crate::__private::codec::decode_header(header)
            }
            fn messages_mut(
                &mut self,
            ) -> &mut $crate::__private::TrackedVec<$crate::__private::Message> {
                &mut self.messages
            }
            fn pages_mut(
                &mut self,
            ) -> &mut $crate::__private::TrackedVec<$crate::__private::PageKind> {
                &mut self.pages
            }
            fn tools_mut(
                &mut self,
            ) -> &mut $crate::__private::TrackedVec<$crate::__private::ToolUse> {
                &mut self.tool_uses
            }
            fn set_summary_index(&mut self, index: $crate::__private::SummaryIndex) {
                self.summary_index = index;
            }
            fn normalize_sidecars(&mut self) {
                $name::normalize_sidecars(self);
            }
        }
    };
}
