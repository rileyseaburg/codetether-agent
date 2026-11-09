# Interactive MSI credential prompt; the password is never written to a transcript.
param([string]$Address, [string]$Model, [bool]$HasToken)
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$form = New-Object Windows.Forms.Form
$form.Text = 'CodeTether Vault setup'; $form.Width = 540; $form.Height = 310
$form.StartPosition = 'CenterScreen'; $form.FormBorderStyle = 'FixedDialog'
$form.MaximizeBox = $false; $form.MinimizeBox = $false
$labels = @('Vault address', 'Vault token (hidden)', 'Default model (optional)')
if ($HasToken) { $labels[1] = 'Vault token (hidden; leave empty to keep configured token)' }
$boxes = @()
for ($i = 0; $i -lt 3; $i++) {
    $label = New-Object Windows.Forms.Label
    $label.Text = $labels[$i]; $label.SetBounds(16, (18 + 58 * $i), 490, 20)
    $box = New-Object Windows.Forms.TextBox
    $box.SetBounds(16, (40 + 58 * $i), 490, 25)
    $form.Controls.Add($label); $form.Controls.Add($box); $boxes += $box
}
$boxes[0].Text = $Address; $boxes[2].Text = $Model
$boxes[1].UseSystemPasswordChar = $true
$note = New-Object Windows.Forms.Label
$note.Text = 'Saved for your Windows user account. Cancel leaves credentials unchanged.'
$note.SetBounds(16, 197, 495, 25); $form.Controls.Add($note)
$save = New-Object Windows.Forms.Button
$save.Text = 'Save'; $save.SetBounds(327, 232, 85, 28)
$save.DialogResult = [Windows.Forms.DialogResult]::OK
$cancel = New-Object Windows.Forms.Button
$cancel.Text = 'Cancel'; $cancel.SetBounds(422, 232, 85, 28)
$cancel.DialogResult = [Windows.Forms.DialogResult]::Cancel
$form.Controls.Add($save); $form.Controls.Add($cancel)
$form.AcceptButton = $save; $form.CancelButton = $cancel
try {
    do {
        if ($form.ShowDialog() -ne [Windows.Forms.DialogResult]::OK) { return $null }
        $valid = $boxes[0].Text.Trim() -and ($HasToken -or $boxes[1].Text.Length)
        if (-not $valid) { [void][Windows.Forms.MessageBox]::Show('Enter a Vault address and token, or choose Cancel.', 'Vault setup') }
    } until ($valid)
    $secure = if ($boxes[1].Text.Length) { ConvertTo-SecureString $boxes[1].Text -AsPlainText -Force } else { New-Object Security.SecureString }
    @{ Address = $boxes[0].Text.Trim(); Token = $secure; Model = $boxes[2].Text.Trim() }
} finally {
    $boxes[1].Clear()
    $form.Dispose()
}