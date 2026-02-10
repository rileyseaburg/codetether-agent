# CodeTether Agent Installer for Windows
# Usage: irm https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.ps1 | iex
#
# Installs the latest release of codetether to a directory on PATH.
# By default, also downloads the FunctionGemma model for local tool-call routing.
# No Rust toolchain required.
#
# Options:
#   -NoFunctionGemma    Skip FunctionGemma model download
#   -FunctionGemmaOnly  Only download the FunctionGemma model (skip binary install)

param(
    [switch]$NoFunctionGemma,
    [switch]$FunctionGemmaOnly,
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

$Repo = 'rileyseaburg/codetether-agent'
$BinaryName = 'codetether'
$InstallDir = Join-Path (Join-Path $env:LOCALAPPDATA 'codetether') 'bin'

# FunctionGemma model configuration
$DataDir = if ($env:XDG_DATA_HOME) { $env:XDG_DATA_HOME } else { Join-Path $env:LOCALAPPDATA 'codetether' }
$FunctionGemmaModelDir = Join-Path (Join-Path $DataDir 'models') 'functiongemma'
$FunctionGemmaModelUrl = 'https://huggingface.co/unsloth/functiongemma-270m-it-GGUF/resolve/main/functiongemma-270m-it-Q8_0.gguf'
$FunctionGemmaModelFile = 'functiongemma-270m-it-Q8_0.gguf'
$FunctionGemmaTokenizerUrl = 'https://huggingface.co/google/functiongemma-270m-it/resolve/main/tokenizer.json'
$FunctionGemmaTokenizerFile = 'tokenizer.json'

function Write-Info  { param([string]$Msg) Write-Host "info: " -ForegroundColor Cyan -NoNewline; Write-Host $Msg }
function Write-Warn  { param([string]$Msg) Write-Host "warn: " -ForegroundColor Yellow -NoNewline; Write-Host $Msg }
function Write-Err   { param([string]$Msg) Write-Host "error: " -ForegroundColor Red -NoNewline; Write-Host $Msg }
function Write-Ok    { param([string]$Msg) Write-Host "  ok: " -ForegroundColor Green -NoNewline; Write-Host $Msg }

function Get-Platform {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        'AMD64' { $archStr = 'x86_64' }
        'ARM64' { $archStr = 'aarch64' }
        default { Write-Err "Unsupported architecture: $arch"; exit 1 }
    }
    return "$archStr-pc-windows-msvc"
}

function Get-LatestVersion {
    $apiUrl = "https://api.github.com/repos/$Repo/releases/latest"
    try {
        $response = Invoke-RestMethod -Uri $apiUrl -Headers @{ 'User-Agent' = 'codetether-installer' }
        return $response.tag_name
    }
    catch {
        Write-Err "Failed to fetch latest release from GitHub"
        exit 1
    }
}

function Install-FunctionGemma {
    Write-Host "`nFunctionGemma Model Setup`n" -NoNewline
    Write-Info "model directory: $FunctionGemmaModelDir"

    if (-not (Test-Path $FunctionGemmaModelDir)) {
        New-Item -ItemType Directory -Path $FunctionGemmaModelDir -Force | Out-Null
    }

    # Download GGUF model
    $modelPath = Join-Path $FunctionGemmaModelDir $FunctionGemmaModelFile
    if (Test-Path $modelPath) {
        Write-Ok "model already exists: $modelPath"
    }
    else {
        Write-Info "downloading FunctionGemma GGUF model (~292 MB)..."
        try {
            Invoke-WebRequest -Uri $FunctionGemmaModelUrl -OutFile $modelPath -UseBasicParsing
            Write-Ok "model downloaded: $modelPath"
        }
        catch {
            Write-Err "failed to download FunctionGemma model"
            Write-Warn "you can retry later: install.ps1 -FunctionGemmaOnly"
            return
        }
    }

    # Download tokenizer (gated model - requires HuggingFace authentication)
    $tokenizerPath = Join-Path $FunctionGemmaModelDir $FunctionGemmaTokenizerFile
    if (Test-Path $tokenizerPath) {
        Write-Ok "tokenizer already exists: $tokenizerPath"
    }
    else {
        $hfToken = $env:HF_TOKEN
        if (-not $hfToken) { $hfToken = $env:HUGGING_FACE_HUB_TOKEN }

        # Check huggingface-cli cached token
        if (-not $hfToken) {
            $hfCacheDir = if ($env:HF_HOME) { $env:HF_HOME } else { Join-Path (Join-Path $env:USERPROFILE '.cache') 'huggingface' }
            $hfCacheToken = Join-Path $hfCacheDir 'token'
            if (Test-Path $hfCacheToken) {
                $hfToken = (Get-Content $hfCacheToken -Raw).Trim()
                if ($hfToken) {
                    Write-Ok "found cached HuggingFace token (from huggingface-cli login)"
                }
            }
        }

        # Interactive prompt
        if (-not $hfToken) {
            Write-Host "`nHuggingFace Authentication Required" -ForegroundColor White
            Write-Host "  The FunctionGemma tokenizer is a gated model that requires"
            Write-Host "  a HuggingFace account with model access granted.`n"
            Write-Host "  Before continuing:"
            Write-Host "  Accept the model license at:"
            Write-Host "  https://huggingface.co/google/functiongemma-270m-it" -ForegroundColor Cyan
            Write-Host ""
            Write-Host "  Choose authentication method:"
            Write-Host "  [1] Open browser to create a token (recommended)" -ForegroundColor White
            Write-Host "  [2] Paste an existing token" -ForegroundColor White
            Write-Host "  [3] Skip tokenizer download" -ForegroundColor White
            Write-Host ""
            $choice = Read-Host "  Choice [1/2/3]"

            switch ($choice) {
                { $_ -eq '1' -or $_ -eq '' } {
                    $tokenUrl = 'https://huggingface.co/settings/tokens/new?tokenType=read&description=codetether-install'
                    Write-Info "opening browser..."
                    Start-Process $tokenUrl
                    Write-Host "  Create a read token, then paste it here."
                    $hfToken = Read-Host "  HuggingFace token"
                }
                '2' {
                    $hfToken = Read-Host "  HuggingFace token"
                }
                '3' {
                    Write-Warn "skipping tokenizer download"
                    Write-Warn "re-run later: `$env:HF_TOKEN='hf_...'; .\install.ps1 -FunctionGemmaOnly"
                    return
                }
            }
        }

        if (-not $hfToken) { $hfToken = '' }
        $hfToken = $hfToken.Trim()

        if (-not $hfToken) {
            Write-Warn "no token provided - skipping tokenizer download"
            Write-Warn "re-run later: `$env:HF_TOKEN='hf_...'; .\install.ps1 -FunctionGemmaOnly"
            return
        }

        Write-Info "downloading tokenizer (authenticated)..."
        try {
            $headers = @{
                'Authorization' = "Bearer $hfToken"
                'User-Agent'    = 'codetether-installer'
            }
            Invoke-WebRequest -Uri $FunctionGemmaTokenizerUrl -OutFile $tokenizerPath -Headers $headers -UseBasicParsing
            if ((Get-Item $tokenizerPath).Length -eq 0) {
                Remove-Item $tokenizerPath -Force
                throw "empty file"
            }
            Write-Ok "tokenizer downloaded: $tokenizerPath"
        }
        catch {
            if (Test-Path $tokenizerPath) { Remove-Item $tokenizerPath -Force }
            Write-Err "failed to download tokenizer (check token and model license access)"
            Write-Warn "1. Accept license: https://huggingface.co/google/functiongemma-270m-it"
            Write-Warn "2. Re-run: `$env:HF_TOKEN='hf_...'; .\install.ps1 -FunctionGemmaOnly"
            return
        }
    }

    Write-Ok "FunctionGemma installed to $FunctionGemmaModelDir"

    # Set persistent user environment variables
    $envVars = @{
        'CODETETHER_TOOL_ROUTER_ENABLED'        = 'true'
        'CODETETHER_TOOL_ROUTER_MODEL_PATH'     = $modelPath
        'CODETETHER_TOOL_ROUTER_TOKENIZER_PATH' = $tokenizerPath
    }

    foreach ($kv in $envVars.GetEnumerator()) {
        [System.Environment]::SetEnvironmentVariable($kv.Key, $kv.Value, 'User')
        Set-Item -Path "Env:$($kv.Key)" -Value $kv.Value
    }

    Write-Ok "FunctionGemma tool-call router is enabled"
    Write-Info "environment variables set for current and future sessions"
}

function Add-ToUserPath {
    param([string]$Dir)

    $currentPath = [System.Environment]::GetEnvironmentVariable('PATH', 'User')
    if ($currentPath -and ($currentPath -split ';' | ForEach-Object { $_.TrimEnd('\') }) -contains $Dir.TrimEnd('\')) {
        return $false
    }

    $newPath = if ($currentPath) { "$Dir;$currentPath" } else { $Dir }
    [System.Environment]::SetEnvironmentVariable('PATH', $newPath, 'User')

    # Also update current session
    if ($env:PATH -notlike "*$Dir*") {
        $env:PATH = "$Dir;$env:PATH"
    }
    return $true
}

# --- Main ---

if ($Help) {
    Write-Host @"
CodeTether Agent Installer for Windows

Usage: .\install.ps1 [OPTIONS]

Options:
  -NoFunctionGemma    Skip FunctionGemma model download
  -FunctionGemmaOnly  Only download the FunctionGemma model
  -Help               Show this help message
"@
    exit 0
}

if ($FunctionGemmaOnly) {
    Install-FunctionGemma
    exit 0
}

Write-Host "`nCodeTether Agent Installer`n" -ForegroundColor White

# Detect platform
$platform = Get-Platform
Write-Info "detected platform: $platform"

# Get latest version
Write-Info "fetching latest release..."
$version = Get-LatestVersion
if (-not $version) {
    Write-Err "could not determine latest version"
    exit 1
}
Write-Info "latest version: $version"

# Build download URL
$artifactName = "codetether-$version-$platform"
$zipName = "$artifactName.zip"
$url = "https://github.com/$Repo/releases/download/$version/$zipName"

# Create temp directory
$tmpDir = Join-Path ([System.IO.Path]::GetTempPath()) "codetether-install-$([guid]::NewGuid().ToString('N').Substring(0,8))"
New-Item -ItemType Directory -Path $tmpDir -Force | Out-Null

try {
    # Download
    Write-Info "downloading $zipName..."
    $zipPath = Join-Path $tmpDir $zipName
    try {
        Invoke-WebRequest -Uri $url -OutFile $zipPath -UseBasicParsing
    }
    catch {
        Write-Err "download failed - no pre-built binary for $platform"
        Write-Err "you can build from source: cargo install codetether-agent"
        exit 1
    }

    # Extract
    Write-Info "extracting..."
    Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force

    # Find the binary (might be in a subfolder or at root)
    $exePath = Get-ChildItem -Path $tmpDir -Filter "$BinaryName.exe" -Recurse | Select-Object -First 1
    if (-not $exePath) {
        # Try the artifact name as the binary name
        $exePath = Get-ChildItem -Path $tmpDir -Filter "$artifactName.exe" -Recurse | Select-Object -First 1
    }
    if (-not $exePath) {
        Write-Err "expected binary not found in archive"
        exit 1
    }

    # Ensure install directory exists
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    # Copy binary
    $destPath = Join-Path $InstallDir "$BinaryName.exe"
    Copy-Item -Path $exePath.FullName -Destination $destPath -Force
    Write-Ok "installed $BinaryName $version to $destPath"

    # Add to PATH if not already there
    $pathAdded = Add-ToUserPath -Dir $InstallDir
    if ($pathAdded) {
        Write-Ok "added $InstallDir to user PATH"
    }

    # Verify installation
    $installed = Get-Command $BinaryName -ErrorAction SilentlyContinue
    if ($installed) {
        $installedVersion = & $BinaryName --version 2>$null
        Write-Ok $installedVersion
    }
    else {
        Write-Warn "$BinaryName may not be available until you restart your terminal"
    }

    Write-Host "`nGet started:" -ForegroundColor White
    Write-Host "  codetether tui       " -ForegroundColor Cyan -NoNewline; Write-Host "- interactive TUI"
    Write-Host "  codetether run `"...`" " -ForegroundColor Cyan -NoNewline; Write-Host "- single prompt"
    Write-Host "  codetether --help    " -ForegroundColor Cyan -NoNewline; Write-Host "- all commands"
    Write-Host ""

    # Install FunctionGemma model
    if (-not $NoFunctionGemma) {
        Install-FunctionGemma
    }
    else {
        Write-Info "skipping FunctionGemma model download (-NoFunctionGemma)"
        Write-Info "you can install it later: .\install.ps1 -FunctionGemmaOnly"
    }
}
finally {
    # Cleanup
    if (Test-Path $tmpDir) {
        Remove-Item -Path $tmpDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}
