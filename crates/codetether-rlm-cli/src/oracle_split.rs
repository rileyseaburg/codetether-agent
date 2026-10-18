//! Optional oracle JSONL dataset writing.
use codetether_cli_options::RlmArgs;
use codetether_rlm as rlm;
pub(super) fn write(
    args: &RlmArgs,
    oracle_result: &rlm::OracleResult,
) -> anyhow::Result<Option<rlm::oracle::SplitWriteStats>> {
    let stats = crate::oracle_stats::stats(oracle_result);
    let split = if let Some(ref out_dir) = args.oracle_out_dir {
        let out_dir_str = out_dir.to_str().ok_or_else(|| {
            anyhow::anyhow!(
                "Oracle output directory path is not valid UTF-8: {}",
                out_dir.display()
            )
        })?;
        Some(stats.write_jsonl_split(out_dir_str, &args.oracle_prefix)?)
    } else {
        None
    };

    Ok(split)
}
