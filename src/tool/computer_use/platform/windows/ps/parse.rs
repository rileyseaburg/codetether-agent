pub fn json_line(lines: &[String]) -> anyhow::Result<serde_json::Value> {
    let line = lines
        .iter()
        .rev()
        .find(|line| !line.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!("PowerShell produced no JSON output"))?;
    Ok(serde_json::from_str(line.trim())?)
}

pub fn escaped_command(script: &str, marker: &str) -> String {
    format!(
        "$ErrorActionPreference='Stop';try{{{script}}}catch{{[pscustomobject]@{{error=$_.Exception.Message}}|ConvertTo-Json -Compress}};Write-Output '{marker}'\n"
    )
}
