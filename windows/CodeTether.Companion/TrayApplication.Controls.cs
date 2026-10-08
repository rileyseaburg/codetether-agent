namespace CodeTether.Companion;

internal sealed partial class TrayApplication
{
    private bool acting;
    private void BindControls()
    {
        form.Pair.Click += async (_, _) => await Act(async () =>
        { await engine.Pair(form.Code.Text.Trim()); form.Code.Clear(); UpdateStatus("Paired — select a monitor, then press Start"); });
        form.Start.Click += async (_, _) => await Act(() =>
        {
            if (!DesktopState.Available()) throw new InvalidOperationException();
            CaptureOptions selection = form.Selection();
            if (selection.RightClick || selection.DoubleClick) mouse.Start();
            engine.Start(selection);
            tray.ShowBalloonTip(4000, "CodeTether monitoring active", "Screenshots of your selected monitor are being sent for AI analysis. Use Pause or Stop in the tray controls.", ToolTipIcon.Info);
            return Task.CompletedTask;
        });
        form.Pause.Click += async (_, _) => await Act(() => engine.Pause());
        form.Stop.Click += async (_, _) => await Act(() => engine.Pause(true));
    }
    private async Task Act(Func<Task> action)
    {
        if (acting) return;
        acting = true; form.UpdateState(engine.Paired, engine.Running, true);
        try { await action(); }
        catch (RelayException exception)
        {
            form.Status.Text = exception.Status is 404 or 410 ? "Code expired or used — create a new session on iOS" : "Pairing rejected — check the code and connection";
        }
        catch (Exception exception) when (exception is HttpRequestException or TaskCanceledException or System.Text.Json.JsonException)
        { form.Status.Text = "Connection unavailable — retry pairing"; }
        catch (Exception exception) when (exception is InvalidDataException or InvalidOperationException or System.ComponentModel.Win32Exception)
        { mouse.Dispose(); form.Status.Text = "Cannot start — check pairing, unlocked desktop and selected monitor"; }
        finally { acting = false; form.UpdateState(engine.Paired, engine.Running); }
    }
}