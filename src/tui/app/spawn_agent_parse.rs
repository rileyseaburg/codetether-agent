//! Argument parsing for the TUI `/spawn` command.

pub(super) struct SpawnArgs {
    pub name: String,
    pub parent: Option<String>,
    pub instructions: String,
}

pub(super) fn parse(rest: &str) -> Option<SpawnArgs> {
    let mut tokens = rest.split_whitespace().peekable();
    let name = tokens.next()?.to_string();
    let parent = if tokens.peek() == Some(&"--parent") {
        tokens.next();
        tokens.next().map(str::to_string)
    } else {
        None
    };
    Some(SpawnArgs {
        name,
        parent,
        instructions: tokens.collect::<Vec<_>>().join(" "),
    })
}
