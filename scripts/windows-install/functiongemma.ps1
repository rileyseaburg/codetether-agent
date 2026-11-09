# Optional legacy local-routing model, unrelated to native Windows OCR.
$data = if ($env:XDG_DATA_HOME) { $env:XDG_DATA_HOME } else { Join-Path $env:LOCALAPPDATA 'codetether' }
$directory = Join-Path $data 'models\functiongemma'
New-Item -ItemType Directory $directory -Force | Out-Null
$files = @{
    'functiongemma-270m-it-Q8_0.gguf' = 'https://huggingface.co/unsloth/functiongemma-270m-it-GGUF/resolve/main/functiongemma-270m-it-Q8_0.gguf'
    'tokenizer.json' = 'https://huggingface.co/google/functiongemma-270m-it/resolve/main/tokenizer.json'
}
foreach ($name in $files.Keys) {
    $destination = Join-Path $directory $name
    if ((Test-Path $destination) -and (Get-Item $destination).Length -gt 0) { continue }
    $staged = "$destination.$([guid]::NewGuid()).download"
    $headers = @{ 'User-Agent' = 'codetether-installer' }
    if ($name -eq 'tokenizer.json' -and $env:HF_TOKEN) { $headers.Authorization = "Bearer $env:HF_TOKEN" }
    try {
        Invoke-WebRequest $files[$name] -OutFile $staged -Headers $headers -UseBasicParsing
        if ((Get-Item $staged).Length -eq 0) { throw 'Empty download.' }
        Move-Item -LiteralPath $staged -Destination $destination
    } catch {
        throw "FUNCTIONGEMMA_DOWNLOAD_FAILED: $name. Partial data retained at $staged. A gated tokenizer requires model access and HF_TOKEN; no credentials are logged or persisted."
    }
}
Write-Host "Optional FunctionGemma files: $directory"
