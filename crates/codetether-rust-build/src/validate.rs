use crate::{RustAction, RustArgs};
use anyhow::{Result, ensure};

pub(crate) fn validate(args: &RustArgs) -> Result<()> {
    for (label, value) in [
        ("toolchain", Some(args.toolchain.as_str())),
        ("package", args.package.as_deref()),
        ("bin", args.bin.as_deref()),
    ] {
        if let Some(value) = value {
            ensure!(
                safe_name(value),
                "invalid {label}: use an alphanumeric first character and only letters, digits, '.', '_' or '-' thereafter"
            );
        }
    }
    ensure!(!args.cwd.as_os_str().is_empty(), "cwd cannot be empty");
    if matches!(
        args.action,
        RustAction::CacheBuild | RustAction::CachePublish
    ) {
        ensure!(
            args.toolchain == "stable",
            "cache actions use the repository toolchain; omit --toolchain"
        );
    }
    let build = matches!(
        args.action,
        RustAction::Build | RustAction::Run | RustAction::Test | RustAction::Check
    );
    ensure!(
        build || (args.package.is_none() && args.bin.is_none()),
        "package/bin selectors require build, run, test or check"
    );
    ensure!(
        args.args.is_empty() || matches!(args.action, RustAction::Run | RustAction::Test),
        "trailing arguments require run or test"
    );
    ensure!(
        args.args.iter().all(|arg| !arg.contains('\0')),
        "arguments cannot contain NUL"
    );
    Ok(())
}

fn safe_name(value: &str) -> bool {
    value
        .as_bytes()
        .first()
        .is_some_and(u8::is_ascii_alphanumeric)
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
}

pub(crate) fn version_matches(version: &str) -> bool {
    let mut words = version.split_whitespace();
    words.next() == Some("rustc") && words.next() == Some("1.99.0")
}
