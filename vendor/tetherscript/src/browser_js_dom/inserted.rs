use super::*;

pub(super) fn connect(parent: &DomHandle, start: usize, count: usize) -> Result<(), String> {
    for offset in 0..count {
        let mut path = parent.path.clone();
        path.push(start + offset);
        custom_host::connected(&DomHandle {
            root: parent.root.clone(),
            path,
        })?;
    }
    Ok(())
}
