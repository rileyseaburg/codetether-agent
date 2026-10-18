//! Resolve positional or flag-form subprocess command arguments.
use codetether_cli_options::McpArgs;
/// Resolve positional command tokens before the whitespace-split flag form.
/// # Arguments
/// * `args` - Parsed command options.
/// # Returns
/// Executable and arguments, or None when neither form supplies a command.
/// # Examples
/// ```no_run
/// # fn example(args: &codetether_cli_options::McpArgs) {
/// let _ = codetether_mcp_cli::resolve_command(args);
/// # }
/// ```
pub fn resolve_command(args: &McpArgs) -> Option<(String, Vec<String>)> {
    let resolved_command: Option<(String, Vec<String>)> = {
        if !args.command_args.is_empty() {
            let mut parts = args.command_args.iter().cloned();
            let cmd = parts.next().unwrap();
            Some((cmd, parts.collect()))
        } else if let Some(s) = args.command.as_ref() {
            let parts: Vec<String> = s.split_whitespace().map(str::to_string).collect();
            if parts.is_empty() {
                None
            } else {
                let mut it = parts.into_iter();
                let cmd = it.next().unwrap();
                Some((cmd, it.collect()))
            }
        } else {
            None
        }
    };
    resolved_command
}
