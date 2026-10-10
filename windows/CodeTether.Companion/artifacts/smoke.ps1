$exe = 'C:\Users\riley\companion-build\target\debug\codetether-companion.exe'
$log = 'C:\Users\riley\companion-build\smoke.txt'
Add-Type @"
using System; using System.Text; using System.Runtime.InteropServices;
public static class W {
  public delegate bool P(IntPtr h, IntPtr l);
  [DllImport("user32.dll")] public static extern bool EnumWindows(P p, IntPtr l);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetClassName(IntPtr h, StringBuilder s, int n);
}
"@
$p = Start-Process $exe -PassThru
Start-Sleep 5
$out = @("pid=$($p.Id) exited=$($p.HasExited)")
$cb = [W+P]{ param($h,$l) $id=0; [W]::GetWindowThreadProcessId($h,[ref]$id) | Out-Null
  if ($id -eq $p.Id) { $t=New-Object Text.StringBuilder 256; $c=New-Object Text.StringBuilder 256
    [W]::GetWindowText($h,$t,256)|Out-Null; [W]::GetClassName($h,$c,256)|Out-Null
    $script:out += "hwnd=$h visible=$([W]::IsWindowVisible($h)) class=$c title=$t" }; $true }
[W]::EnumWindows($cb,[IntPtr]::Zero) | Out-Null
# Second launch must hit the single-instance guard and exit.
$q = Start-Process $exe -PassThru; Start-Sleep 3
$out += "second_instance_exited=$($q.HasExited)"
$cb2 = [W+P]{ param($h,$l) $id=0; [W]::GetWindowThreadProcessId($h,[ref]$id) | Out-Null
  if ($id -eq $q.Id -and [W]::IsWindowVisible($h)) { $t=New-Object Text.StringBuilder 256; $c=New-Object Text.StringBuilder 256
    [W]::GetWindowText($h,$t,256)|Out-Null; [W]::GetClassName($h,$c,256)|Out-Null
    $script:out += "second hwnd=$h class=$c title=$t" }; $true }
[W]::EnumWindows($cb2,[IntPtr]::Zero) | Out-Null
if (-not $q.HasExited) { Stop-Process -Id $q.Id -Force }
Get-Process codetether-companion -ErrorAction SilentlyContinue | Stop-Process -Force
$out | Set-Content $log
