use super::super::task_input::TaskInput;
use std::sync::Mutex;

pub(super) static LOCK: Mutex<()> = Mutex::new(());

pub(super) fn task(name: &str) -> TaskInput {
    TaskInput {
        id: None,
        name: name.into(),
        instruction: "Inspect the diff".into(),
        specialty: None,
    }
}
