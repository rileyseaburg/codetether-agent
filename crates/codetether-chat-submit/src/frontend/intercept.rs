//! Frontend intercept operations for chat submission.
/// Implementation detail of [crate::declare_frontend_chat_submit].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_chat_intercept {
    ($goal_command:path, $goal_answer:path, $feedback:path) => {
        mod intercept {
            use super::Native;
            impl $crate::Intercept for Native<'_> {
                async fn goal_command(&mut self) -> bool {
                    $goal_command(self.app, self.slot, self.runtime).await
                }
                async fn goal_answer(&mut self) -> bool {
                    $goal_answer(self.app, self.slot, self.runtime).await
                }
                fn approval_feedback(&mut self) -> bool {
                    $feedback(self.app)
                }
            }
        }
    };
}
