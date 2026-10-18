//! Ralph create command.
use codetether_cli_options::RalphArgs;
use codetether_ralph_core as ralph;
pub(super) async fn execute(args: RalphArgs) -> anyhow::Result<()> {
    let project = args.project_name.as_deref().unwrap_or("my-project");
    let feature = args.feature.as_deref().unwrap_or("new-feature");
    let prd = ralph::create_prd_template(project, feature);

    if args.json {
        println!("{}", serde_json::to_string_pretty(&prd)?);
    } else {
        println!("{}", serde_json::to_string_pretty(&prd)?);
        eprintln!("\nSave this to prd.json and run: codetether ralph run");
    }
    Ok(())
}
