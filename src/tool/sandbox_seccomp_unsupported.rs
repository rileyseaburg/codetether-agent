use anyhow::Result;

#[derive(Debug)]
pub(crate) struct Program;

impl Program {
    pub(crate) fn fd(&self) -> i32 {
        -1
    }
}

pub(crate) fn prepare(_allow_network: bool) -> Result<Option<Program>> {
    Ok(None)
}
