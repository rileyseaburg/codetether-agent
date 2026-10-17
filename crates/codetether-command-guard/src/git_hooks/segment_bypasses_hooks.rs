//! segment bypasses hooks in shell hook-bypass detection.
use super::*;

pub(super) fn segment_bypasses_hooks(segment: &str) -> bool {
    let words: Vec<&str> = segment.split_whitespace().map(clean).collect();
    let Some(git) = words.iter().position(|word| executable(word) == "git") else {
        return false;
    };
    if words[..git].iter().any(|word| hook_skip_env(word)) {
        return true;
    }
    let args = &words[git + 1..];
    let mut index = 0;
    while index < args.len() {
        let arg = args[index];
        if arg == "-c" {
            if args.get(index + 1).is_some_and(|value| hooks_config(value)) {
                return true;
            }
            index += 2;
            continue;
        }
        if let Some(value) = arg.strip_prefix("--config-env=")
            && hooks_config(value)
        {
            return true;
        }
        if !arg.starts_with('-') {
            return subcommand_bypasses(arg, &args[index + 1..]);
        }
        // Global options taking a separate value, e.g. `-C dir`.
        index += if matches!(arg, "-C" | "--git-dir" | "--work-tree") {
            2
        } else {
            1
        };
    }
    false
}
