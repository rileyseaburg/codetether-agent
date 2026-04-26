use crate::tool::computer_use::input::ComputerUseInput;

pub async fn handle_scroll(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let amount = input.scroll_amount.unwrap_or(-120);
    let script = format!(
        "Add-Type -MemberDefinition '[DllImport(\"user32.dll\")] public static extern void mouse_event(int f,int dx,int dy,int d,int e);' -Name U -Namespace W;[W.U]::mouse_event(2048,0,0,{amount},0);@{{scrolled={amount}}}|ConvertTo-Json -Compress"
    );
    Ok(super::super::response::success_result(
        super::super::ps::run(&script).await?,
    ))
}
