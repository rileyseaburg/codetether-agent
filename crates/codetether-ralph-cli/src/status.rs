//! Ralph status command.
use codetether_cli_options::RalphArgs;
use codetether_ralph_core as ralph;
pub(super) async fn execute(args: RalphArgs) -> anyhow::Result<()> {
    let prd = ralph::Prd::load(&args.prd).await?;
    let stories: Vec<String> = prd
        .user_stories
        .iter()
        .map(|s| {
            let check = if s.passes { "[x]" } else { "[ ]" };
            format!("- {} {}: {}", check, s.id, s.title)
        })
        .collect();

    println!("# Ralph Status\n");
    println!("**Project:** {}", prd.project);
    println!("**Feature:** {}", prd.feature);
    println!("**Branch:** {}", prd.branch_name);
    println!(
        "**Progress:** {}/{} stories\n",
        prd.passed_count(),
        prd.user_stories.len()
    );
    println!("## Stories\n{}", stories.join("\n"));
    Ok(())
}
