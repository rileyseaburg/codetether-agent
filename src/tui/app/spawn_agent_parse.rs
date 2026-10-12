//! Argument parsing for the TUI `/spawn` command.

pub(super) struct SpawnArgs {
    pub name: String,
    pub parent: Option<String>,
    pub instructions: String,
}

pub(super) fn parse(rest: &str) -> Option<SpawnArgs> {
    let mut tokens = rest.split_whitespace();
    let name = tokens.next()?.to_string();
    let mut parent = None;
    let mut instruction_parts = Vec::new();
    while let Some(token) = tokens.next() {
        if token == "--parent" {
            parent = Some(tokens.next().unwrap_or_default().to_string());
            instruction_parts.extend(tokens);
            break;
        }
        instruction_parts.push(token);
    }
    Some(SpawnArgs {
        name,
        parent,
        instructions: instruction_parts.join(" "),
    })
}

#[cfg(test)]
#[path = "spawn_agent_parse_tests.rs"]
mod tests;
