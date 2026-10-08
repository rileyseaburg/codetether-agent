namespace CodeTether.Companion;

internal static class Program
{
    [STAThread]
    private static void Main()
    {
        using Mutex instance = new(true, "Local\\CodeTether.ScreenCompanion", out bool created);
        if (!created) { MessageBox.Show("CodeTether is already running in the notification area."); return; }
        ApplicationConfiguration.Initialize();
        using TrayApplication application = new();
        Application.Run(application);
        GC.KeepAlive(instance);
    }
}