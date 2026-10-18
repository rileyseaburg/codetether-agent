//! Frontend goal dispatch, task reads and editor selection.
/// Implementation detail of [crate::declare_frontend_goal_input].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_goal_input_actions {
    ($state:path; $goal:path, $tasks:path, $editor:path) => {
        mod actions {
            use super::Native;
            use $state as TaskState;
            impl $crate::input::Actions for Native<'_> {
                fn session_id(&self) -> String {
                    self.slot.view().id().to_string()
                }
                async fn goal(&mut self, id: &str, text: &str) -> bool {
                    $goal(self.app, id, text).await
                }
                fn tasks(&self, id: &str) -> anyhow::Result<TaskState> {
                    $tasks(id)
                }
                fn edit(&mut self, id: &str) {
                    $editor(self.app, id);
                }
                fn restart(&mut self, id: &str) {
                    super::super::resume::restart(self.app, id, self.runtime);
                }
            }
        }
    };
}
