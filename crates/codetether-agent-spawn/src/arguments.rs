//! Parse slash-command arguments for a managed agent spawn.
//! SpawnArgs retains the requested name, optional parent and explicit instructions.

/// Parsed spawn command values; hosts decide whether parent selection is supported.
///
/// # Examples
/// ```
/// let args = codetether_agent_spawn::parse_spawn_args("reviewer audit").unwrap();
/// assert_eq!(args.name, "reviewer");
/// ```

pub struct SpawnArgs {
    /// Requested agent name.
    pub name: String,
    /// Parent argument, including an empty value when the flag is incomplete.
    pub parent: Option<String>,
    /// Remaining whitespace-normalized instruction words.
    pub instructions: String,
}
/// Parse a name followed by instructions and an optional --parent flag.
///
/// # Arguments
/// * `rest` — Arguments following the spawn command.
///
/// # Returns
/// None when no name exists; otherwise the parsed request.
///
/// # Examples
/// ```
/// assert!(codetether_agent_spawn::parse_spawn_args("").is_none());
/// ```

pub fn parse_spawn_args(rest: &str) -> Option<SpawnArgs> {
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
