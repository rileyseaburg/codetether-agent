//! Snapshot implementation for the shared session schema.
/// Internal storage binding for application-local session types.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_storage_snapshot {
    ($name:ident) => {
        impl $crate::__private::store::Snapshot for $name {
            type Tool = $crate::__private::ToolUse;
            fn id(&self) -> &str {
                &self.id
            }
            fn state(&self) -> &$crate::__private::State {
                &self.storage
            }
            fn messages(&self) -> &$crate::__private::TrackedVec<$crate::__private::Message> {
                &self.messages
            }
            fn pages(&self) -> &$crate::__private::TrackedVec<$crate::__private::PageKind> {
                &self.pages
            }
            fn tools(&self) -> &$crate::__private::TrackedVec<$crate::__private::ToolUse> {
                &self.tool_uses
            }
            fn encode_header(&self) -> $crate::__private::anyhow::Result<::std::string::String> {
                $crate::__private::codec::encode_header($crate::header(self))
            }
        }
    };
}
