//! Interactive Ralph arguments behavior.

/// Parse positional / flag arguments for `/ralph run`.
///
/// Accepts: `[prd_path] [--iters N]` in either order.
/// # Arguments
/// * `args` - Positional PRD path and optional iteration-limit flags.
/// # Returns
/// The first PRD path and iteration limit, defaulting to ten.
/// # Examples
/// ~~~
/// assert_eq!(codetether_ralph_cli::interactive::parse_ralph_run_args(&["work.json", "--iters", "3"]), (Some("work.json"), 3));
/// ~~~
pub fn parse_ralph_run_args<'a>(args: &[&'a str]) -> (Option<&'a str>, usize) {
    let mut prd: Option<&str> = None;
    let mut iters: usize = 10;
    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "--iters" | "--max-iterations" => {
                if let Some(v) = args.get(i + 1).and_then(|s| s.parse().ok()) {
                    iters = v;
                    i += 2;
                    continue;
                }
            }
            other if !other.starts_with("--") && prd.is_none() => {
                prd = Some(other);
            }
            _ => {}
        }
        i += 1;
    }
    (prd, iters)
}
