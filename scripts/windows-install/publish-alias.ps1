# Called ONLY after registration and activation; preserve the legacy binary as evidence.
param([string]$AliasDir, [string]$Work)
$legacy = Join-Path $env:LOCALAPPDATA 'codetether\bin\codetether.exe'
$machinePath = [Environment]::GetEnvironmentVariable('PATH', 'Machine')
foreach ($directory in ($machinePath -split ';')) {
    $directory = [Environment]::ExpandEnvironmentVariables($directory.Trim().Trim('"'))
    if ($directory -and (Test-Path (Join-Path $directory 'codetether.exe')) -and
        (Join-Path $directory 'codetether.exe') -ine $legacy -and $directory -ine $AliasDir) {
        throw "PATH_CONFLICT: Machine PATH contains $directory\codetether.exe. Package activated, but PATH was not changed; use $AliasDir\codetether.exe explicitly."
    }
}
$oldPath = $env:PATH
$userPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
$changed = $false
$env:PATH = "$AliasDir;$oldPath"
try {
    $command = Get-Command codetether.exe -ErrorAction Stop
    if ($command.CommandType -ne 'Application' -or $command.Source -ine (Join-Path $AliasDir 'codetether.exe')) { throw 'PATH_CONFLICT: A command shadows the packaged alias.' }
    $userPath | Set-Content (Join-Path $Work 'previous-user-path.txt')
    $remaining = @($userPath -split ';' | Where-Object { $_ -and $_.TrimEnd('\') -ine $AliasDir.TrimEnd('\') })
    [Environment]::SetEnvironmentVariable('PATH', (@($AliasDir) + $remaining -join ';'), 'User')
    $changed = $true
    if (Test-Path -LiteralPath $legacy) { Move-Item -LiteralPath $legacy -Destination (Join-Path $Work 'previous-codetether.exe') }
} catch {
    $env:PATH = $oldPath
    if ($changed) { [Environment]::SetEnvironmentVariable('PATH', $userPath, 'User') }
    throw
}
Write-Host "Packaged command: $AliasDir\codetether.exe. Open a new terminal for the persisted PATH."
