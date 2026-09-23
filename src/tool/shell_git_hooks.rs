//! Detection of shell Git invocations that would skip repository hooks.
//!
//! Commits and pushes must run the repository's pre-commit, post-commit and
//! pre-push hooks. `--no-verify` (and commit's `-n`) and hook-path overrides
//! bypass them, so they are refused outright.

pub(super) fn detected(command: &str) -> bool {
    command
        .split([';', '|', '&', '\n', '\r'])
        .any(segment_bypasses_hooks)
}

fn segment_bypasses_hooks(segment: &str) -> bool {
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
        index += if matches!(arg, "-C" | "--git-dir" | "--work-tree") { 2 } else { 1 };
    }
    false
}

fn subcommand_bypasses(sub: &str, rest: &[&str]) -> bool {
    match sub {
        "commit" => rest.iter().any(|arg| *arg == "--no-verify" || short_n(arg)),
        "merge" | "push" | "cherry-pick" | "revert" | "am" | "rebase" => {
            rest.iter().any(|arg| *arg == "--no-verify")
        }
        "config" => rest.iter().any(|arg| hooks_config(arg)),
        _ => false,
    }
}

/// Commit's `-n` is `--no-verify`, alone or inside a short-flag group (`-an`).
/// Stop at `-m`/`-F`/`-C`, whose value may itself contain an `n`.
fn short_n(arg: &str) -> bool {
    let Some(flags) = arg.strip_prefix('-') else {
        return false;
    };
    if flags.is_empty() || flags.starts_with('-') {
        return false;
    }
    for flag in flags.chars() {
        match flag {
            'n' => return true,
            'm' | 'F' | 'C' | 'c' | 't' | 'S' | 'u' => return false,
            c if c.is_ascii_alphabetic() => continue,
            _ => return false,
        }
    }
    false
}

fn hooks_config(value: &str) -> bool {
    value.to_ascii_lowercase().starts_with("core.hookspath")
}

/// Environment assignments that make repository hook managers skip checks.
fn hook_skip_env(word: &str) -> bool {
    let Some((name, _)) = word.split_once('=') else {
        return false;
    };
    matches!(name, "SKIP" | "PRE_COMMIT_ALLOW_NO_CONFIG" | "HUSKY" | "LEFTHOOK")
        || name.starts_with("GIT_CONFIG_KEY_")
        || name == "GIT_CONFIG_PARAMETERS"
}

fn executable(word: &str) -> &str {
    word.rsplit('/').next().unwrap_or("")
}

fn clean(word: &str) -> &str {
    word.trim_matches(['\'', '"', '(', ')'])
}
