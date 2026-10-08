using Microsoft.Win32;

namespace CodeTether.Companion;

/// <summary>Owns the visible tray indicator and the local monitoring lifecycle.</summary>
internal sealed partial class TrayApplication : ApplicationContext
{
    private readonly MainForm form = new();
    private readonly CaptureEngine engine = new();
    private readonly NotifyIcon tray = new() { Icon = SystemIcons.Information, Text = "CodeTether — capture off", Visible = true };
    private readonly System.Windows.Forms.Timer timer = new() { Interval = 500 };
    private readonly MouseHook mouse;
    private bool exiting;
    public TrayApplication()
    {
        mouse = new MouseHook(engine.Trigger);
        ContextMenuStrip menu = new();
        menu.Items.Add("Open monitoring controls", null, (_, _) => form.ShowWindow());
        menu.Items.Add("Pause capture", null, async (_, _) => await Act(() => engine.Pause()));
        menu.Items.Add("Stop / unpair", null, async (_, _) => await Act(() => engine.Pause(true)));
        menu.Items.Add("Exit", null, async (_, _) => { await Act(() => engine.Pause(true)); ExitThread(); });
        tray.ContextMenuStrip = menu; tray.DoubleClick += (_, _) => form.ShowWindow();
        engine.Status += UpdateStatus; BindControls();
        timer.Tick += async (_, _) => await engine.Tick(); timer.Start();
        SystemEvents.SessionSwitch += SessionChanged;
        form.FormClosing += (_, args) => { if (!exiting) { args.Cancel = true; form.Hide(); } };
        form.Show();
    }
    private void UpdateStatus(string text)
    {
        if (!engine.Running) mouse.Dispose();
        form.Status.Text = text; form.UpdateState(engine.Paired, engine.Running, acting);
        tray.Icon = engine.Running ? SystemIcons.Shield : SystemIcons.Information;
        tray.Text = engine.Running ? "CodeTether — monitoring ACTIVE" : "CodeTether — capture off";
    }
    private void SessionChanged(object sender, SessionSwitchEventArgs args)
    {
        if (args.Reason is SessionSwitchReason.SessionLock or SessionSwitchReason.SessionLogoff or SessionSwitchReason.ConsoleDisconnect or SessionSwitchReason.RemoteDisconnect)
            form.BeginInvoke(new Action(async () => await Act(() => engine.Pause())));
    }
    protected override void ExitThreadCore()
    { exiting = true; timer.Stop(); SystemEvents.SessionSwitch -= SessionChanged; mouse.Dispose(); engine.Dispose(); tray.Dispose(); form.Dispose(); timer.Dispose(); base.ExitThreadCore(); }
}