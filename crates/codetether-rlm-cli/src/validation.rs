//! CLI validation performed before reading input or initializing providers.
use codetether_cli_options::RlmArgs;
pub(super) fn validate(args: &RlmArgs) -> anyhow::Result<()> {
    if args.consensus_runs == 0 {
        anyhow::bail!("--consensus-runs must be at least 1");
    }
    if !(0.0..=1.0).contains(&args.consensus_threshold) {
        anyhow::bail!("--consensus-threshold must be between 0.0 and 1.0");
    }
    if args.oracle_verify {
        eprintln!(
            "warning: --oracle-verify is deprecated, oracle verification is now always-on. \
             Use --no-oracle-verify to disable."
        );
    }

    Ok(())
}
