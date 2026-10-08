namespace CodeTether.Companion;

internal sealed partial class MainForm
{
    internal void UpdateState(bool paired, bool running, bool working = false)
    {
        Pair.Enabled = !paired && !working; Code.Enabled = Pair.Enabled;
        Start.Enabled = paired && !running && !working;
        Pause.Enabled = paired && running && !working; Stop.Enabled = paired && !working;
        Monitor.Enabled = !running && !working; Interval.Enabled = Monitor.Enabled;
        Periodic.Enabled = Monitor.Enabled; RightClick.Enabled = Monitor.Enabled; DoubleClick.Enabled = Monitor.Enabled;
    }
    internal void ShowWindow()
    { Show(); WindowState = FormWindowState.Normal; Activate(); }
}