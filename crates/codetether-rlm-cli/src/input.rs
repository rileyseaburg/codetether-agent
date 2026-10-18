//! RLM CLI input handling.

/// Gather file, stdin or inline input.
/// # Arguments
/// * `args` — CLI file paths or inline/stdin content selection.
/// # Returns
/// Selected input content with per-file errors retained as text.
/// # Errors
/// Returns stdin read failures or missing-input errors.
/// # Examples
/// ```no_run
/// # fn example(args: &codetether_cli_options::RlmArgs) {
/// let _ = codetether_rlm_cli::gather_rlm_content(args);
/// # }
/// ```
pub fn gather_rlm_content(args: &codetether_cli_options::RlmArgs) -> anyhow::Result<String> {
    use std::io::Read;

    if !args.file.is_empty() {
        let mut parts = Vec::new();
        for path in &args.file {
            match std::fs::read_to_string(path) {
                Ok(content) => parts.push(format!(
                    "=== FILE: {} ===\n{}\n=== END FILE ===\n",
                    path.display(),
                    content
                )),
                Err(e) => parts.push(format!(
                    "=== FILE: {} ===\nError: {}\n=== END FILE ===\n",
                    path.display(),
                    e
                )),
            }
        }
        return Ok(parts.join("\n"));
    }

    if let Some(ref c) = args.content {
        if c == "-" {
            let mut stdin_content = String::new();
            std::io::stdin().read_to_string(&mut stdin_content)?;
            return Ok(stdin_content);
        }
        return Ok(c.clone());
    }

    anyhow::bail!("Either --file or --content must be provided");
}
