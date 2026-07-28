param([string]$Base = "")

$Limit = 50
if (-not $Base) {
    git rev-parse --verify HEAD~1 *> $null
    $Base = if ($LASTEXITCODE -eq 0) { "HEAD~1" } else { "HEAD" }
}

function Count-Lines($Lines) {
    $Count = 0
    foreach ($Raw in $Lines) {
        $Line = $Raw.TrimStart()
        if ($Line -eq "") { continue }
        if ($Line.StartsWith("//")) { continue }
        if ($Line.StartsWith("/*")) { continue }
        if ($Line.StartsWith("*")) { continue }
        $Count++
    }
    $Count
}

$Files = @()
$RangeFiles = git diff --name-only --diff-filter=ACMR "$Base...HEAD" -- "src/**/*.rs" 2>$null
if ($LASTEXITCODE -eq 0) { $Files += $RangeFiles }
$Files += git diff --name-only --diff-filter=ACMR -- "src/**/*.rs"
$Files += git diff --cached --name-only --diff-filter=ACMR -- "src/**/*.rs"
$Files += git ls-files --others --exclude-standard -- "src/**/*.rs"
$Files = $Files | Where-Object { $_ } | Sort-Object -Unique
$Failed = $false

foreach ($File in $Files) {
    if (-not (Test-Path $File)) { continue }
    $Current = Count-Lines (Get-Content $File)
    $BaseText = git show "${Base}:$File" 2>$null
    $Previous = if ($LASTEXITCODE -eq 0) { Count-Lines $BaseText } else { 0 }
    if ($Current -le $Limit) { continue }
    if ($Previous -gt $Limit -and $Current -le $Previous) { continue }
    Write-Output "$File has $Current effective lines; limit is $Limit, previous was $Previous"
    $Failed = $true
}

if ($Failed) { exit 1 }
