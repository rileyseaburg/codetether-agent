namespace CodeTether.Companion;

/// <summary>Visible local permission and monitor selection; no remote action can press Start.</summary>
internal sealed partial class MainForm : Form
{
    internal readonly TextBox Code = new() { MaxLength = 12, PlaceholderText = "12-character code from iOS" };
    internal readonly ComboBox Monitor = new() { DropDownStyle = ComboBoxStyle.DropDownList, Width = 400 };
    internal readonly NumericUpDown Interval = new() { Minimum = 15, Maximum = 300, Value = 30, Width = 80 };
    internal readonly CheckBox Periodic = new() { Text = "Automatic screenshots", Checked = true, AutoSize = true };
    internal readonly CheckBox RightClick = new() { Text = "Right-click screenshots", AutoSize = true, Checked = true };
    internal readonly CheckBox DoubleClick = new() { Text = "Double-click screenshots", AutoSize = true, Checked = true };
    internal readonly Button Pair = new() { Text = "Pair", AutoSize = true };
    internal readonly Button Start = new() { Text = "Start / resume", AutoSize = true, Enabled = false };
    internal readonly Button Pause = new() { Text = "Pause", AutoSize = true, Enabled = false };
    internal readonly Button Stop = new() { Text = "Stop / unpair", AutoSize = true, Enabled = false };
    internal readonly Label Status = new() { AutoSize = true, MaximumSize = new Size(510, 0), Text = "Not paired — capture is off" };
    public MainForm()
    {
        Text = "CodeTether — Screen companion"; ClientSize = new Size(570, 455);
        MinimumSize = Size; StartPosition = FormStartPosition.CenterScreen;
        FlowLayoutPanel layout = new() { Dock = DockStyle.Fill, FlowDirection = FlowDirection.TopDown, WrapContents = false, Padding = new Padding(20), AutoScroll = true };
        Controls.Add(layout);
        Label notice = new() { AutoSize = true, MaximumSize = new Size(510, 0), Text =
            "Company screen monitoring. Start permits screenshots of the selected monitor for AI analysis, including iOS requests. The tray icon stays visible. Pause or Stop anytime. No audio, keyboard recording, or remote control." };
        layout.Controls.Add(notice); layout.Controls.Add(Row(Code, Pair));
        foreach (Screen screen in Screen.AllScreens) Monitor.Items.Add(new MonitorChoice(screen.DeviceName, screen.Bounds));
        Monitor.SelectedIndex = 0; layout.Controls.Add(Monitor);
        layout.Controls.Add(Row(Periodic, new Label { Text = "Seconds:", AutoSize = true }, Interval));
        layout.Controls.Add(RightClick); layout.Controls.Add(DoubleClick);
        layout.Controls.Add(Row(Start, Pause, Stop)); layout.Controls.Add(Status);
        layout.Controls.Add(new Label { AutoSize = true, MaximumSize = new Size(510, 0), Text =
            "Screenshots leave this computer for your selected AI provider. Credentials and screenshots are not saved here. Sessions expire after one hour or 120 captures. Closing this window minimizes to the tray; use Stop or Exit to end capture." });
    }
    private static FlowLayoutPanel Row(params Control[] controls)
    { FlowLayoutPanel row = new() { AutoSize = true, WrapContents = false }; row.Controls.AddRange(controls); return row; }
    internal CaptureOptions Selection()
    {
        MonitorChoice choice = (MonitorChoice)(Monitor.SelectedItem ?? throw new InvalidOperationException());
        return new(choice.Bounds, (int)Interval.Value, Periodic.Checked, RightClick.Checked, DoubleClick.Checked);
    }
    private sealed record MonitorChoice(string Name, Rectangle Bounds)
    { public override string ToString() => $"{Name} — {Bounds.Width}×{Bounds.Height}"; }
}