use anyhow::Result;

#[derive(Debug)]
pub(crate) struct Program;

impl Program {
    pub(crate) fn fd(&self) -> i32 {
        -1
}

pub(crate) fn apply(_command: &mut tokio::process::Command, _program: &Program) {}

pub(crate) fn apply_std(
    _command: &mut std::process::Command,
    _program: &Program,
) {
}
}

pub(crate) fn prepare(_allow_network: bool) -> Result<Option<Program>> {
    Ok(None)
}