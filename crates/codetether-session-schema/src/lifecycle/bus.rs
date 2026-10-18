//! bus operations on the application-local shared session schema.
/// Internal implementation of the shared session lifecycle declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_lifecycle_bus {
    ($name:ident) => {
        impl $name {
            /// Attach an agent bus for publishing agent thinking/reasoning events.
            pub fn with_bus(mut self, bus: ::std::sync::Arc<$crate::__private::AgentBus>) -> Self {
                self.bus = Some(bus);
                self
            }

            /// Reattach the process-global bus after loading a session from disk.
            pub(crate) fn attach_global_bus_if_missing(&mut self) {
                if self.bus.is_none() {
                    self.bus = $crate::__private::global();
                }
            }

            /// Rebuild runtime state omitted from persisted or legacy sessions.
            pub(crate) fn normalize_sidecars(&mut self) {
                self.attach_global_bus_if_missing();
                $crate::__private::metadata::lifecycle::hydrate(
                    &mut self.metadata,
                    &self.messages,
                    &mut self.pages,
                );
            }
        }
    };
}
