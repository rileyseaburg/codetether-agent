//! Application usage state for shared chat events.
/// Supporting implementation for [crate::declare_chat_state].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_chat_usage {
    ($native:ident) => {
        impl $crate::UsageView for $native<'_> {
            fn set_model(&mut self, model: String) {
                self.0.state.last_completion_model = Some(model);
            }
            fn set_latency(&mut self, duration_ms: u64) {
                self.0.state.last_completion_latency_ms = Some(duration_ms);
            }
            fn set_prompt_tokens(&mut self, tokens: usize) {
                self.0.state.last_completion_prompt_tokens = Some(tokens);
            }
            fn set_output_tokens(&mut self, tokens: usize) {
                self.0.state.last_completion_output_tokens = Some(tokens);
            }
            fn messages(&mut self) -> &mut Vec<$crate::__private::ChatMessage> {
                &mut self.0.state.messages
            }
            fn set_status(&mut self, status: String) {
                self.0.state.status = status;
            }
        }
    };
}
