use crate::swarm::subtask::SubTask;

pub(super) fn prepare(
    tasks: &[(String, String, Option<String>)],
) -> (Vec<(String, SubTask)>, Vec<bool>, Vec<bool>) {
    let mut prepared = Vec::with_capacity(tasks.len());
    let mut expects_changes = Vec::with_capacity(tasks.len());
    let mut verification = Vec::with_capacity(tasks.len());
    for (name, instruction, specialty) in tasks {
        let mut task = SubTask::new(name, instruction);
        task.specialty.clone_from(specialty);
        expects_changes.push(task.expects_file_changes());
        verification.push(task.is_verification());
        prepared.push((name.clone(), task));
    }
    (prepared, expects_changes, verification)
}
