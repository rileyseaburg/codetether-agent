//! Frontend journal operations for the goal editor.
/// Implementation detail of [crate::declare_frontend_goal_editor].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_goal_editor_journal {
    ($state:path, $edit:path; $read:path, $update:path) => {
        mod journal {
            use $edit as GoalEdit;
            use $state as TaskState;
            pub(crate) struct Native;
            impl $crate::Journal for Native {
                fn read(id: &str) -> anyhow::Result<TaskState> {
                    $read(id)
                }
                async fn update(id: &str, edit: GoalEdit) -> anyhow::Result<()> {
                    $update(id, edit).await?;
                    Ok(())
                }
            }
        }
    };
}
