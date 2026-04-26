//! Windows snapshot handling for computer use.

pub async fn handle_snapshot(
    _input: &crate::tool::computer_use::input::ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let script = r#"
Add-Type -AssemblyName System.Windows.Forms,System.Drawing
$b=[System.Windows.Forms.SystemInformation]::VirtualScreen
$bmp=New-Object System.Drawing.Bitmap $b.Width,$b.Height
$g=[System.Drawing.Graphics]::FromImage($bmp)
$g.CopyFromScreen($b.Location,[System.Drawing.Point]::Empty,$b.Size)
$path=Join-Path ([System.IO.Path]::GetTempPath()) ("codetether-snapshot-" + [System.Guid]::NewGuid().ToString() + ".png")
$bmp.Save($path,[System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose();$bmp.Dispose()
[pscustomobject]@{captured=$true;mime_type="image/png";path=$path;width=$b.Width;height=$b.Height;left=$b.Left;top=$b.Top}|ConvertTo-Json -Compress
"#;
    let metadata = super::ps::run(script).await?;
    Ok(super::response::success_result(metadata))
}
