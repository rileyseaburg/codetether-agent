# Organization/signing restrictions must stop before copying or unblocking scripts.
param([string]$Command, [string]$Work)
function Get-ExecutionPolicy { param([switch]$List) $policy }
function Unblock-File { throw 'UNEXPECTED_UNBLOCK' }
function Copy-Item { throw 'UNEXPECTED_COPY' }
function New-Item { throw 'UNEXPECTED_STAGE' }
foreach ($scope in @('MachinePolicy', 'UserPolicy', 'Process', 'CurrentUser', 'LocalMachine')) {
    $policy = @([pscustomobject]@{ Scope = $scope; ExecutionPolicy = 'AllSigned' })
    $log = Join-Path $Work "policy-$scope-AllSigned.log"
    $result = & ([scriptblock]::Create($Command)) 2> $log
    Assert-Contract ($result -eq 1) "AllSigned $scope rejected"
    Assert-Contract ((Get-Content -LiteralPath $log -Raw) -match 'SIGNED_INSTALLER_REQUIRED') 'typed signing rejection'
}
foreach ($scope in @('MachinePolicy', 'UserPolicy')) {
    $policy = @([pscustomobject]@{ Scope = $scope; ExecutionPolicy = 'Restricted' })
    $log = Join-Path $Work "policy-$scope-Restricted.log"
    $result = & ([scriptblock]::Create($Command)) 2> $log
    Assert-Contract ($result -eq 1) "organization Restricted $scope rejected"
    Assert-Contract ((Get-Content -LiteralPath $log -Raw) -match 'SIGNED_INSTALLER_REQUIRED') 'typed organization rejection'
}
$savedPolicy = $env:CODETETHER_PARENT_EXECUTION_POLICY
try {
    $env:CODETETHER_PARENT_EXECUTION_POLICY = 'AllSigned'
    $policy = @([pscustomobject]@{ Scope = 'Process'; ExecutionPolicy = 'RemoteSigned' })
    $log = Join-Path $Work 'policy-inherited-process-AllSigned.log'
    $result = & ([scriptblock]::Create($Command)) 2> $log
    Assert-Contract ($result -eq 1) 'inherited AllSigned cannot be hidden by process RemoteSigned'
    Assert-Contract ((Get-Content -LiteralPath $log -Raw) -match 'SIGNED_INSTALLER_REQUIRED') 'typed inherited signing rejection'
} finally { $env:CODETETHER_PARENT_EXECUTION_POLICY = $savedPolicy }
