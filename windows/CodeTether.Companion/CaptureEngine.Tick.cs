namespace CodeTether.Companion;

internal sealed partial class CaptureEngine
{
    public async Task Tick()
    {
        if (!Running || busy || pair is null || options is null) return;
        if (pair.ExpiresAt <= DateTimeOffset.UtcNow) { await Pause(true); return; }
        busy = true;
        CancellationToken token = lifetime.Token;
        try
        {
            if (DateTimeOffset.UtcNow >= nextPoll)
            {
                DeviceCommand command = await client.Poll(pair, token);
                token.ThrowIfCancellationRequested(); schedule.RequestId = command.RequestId;
                nextPoll = DateTimeOffset.UtcNow.AddSeconds(2);
            }
            string? trigger = schedule.Due(DateTimeOffset.UtcNow, options.Periodic, options.Interval);
            if (trigger is null) return;
            if (!DesktopState.Available()) { await Pause(); return; }
            FrameBody frame = Screenshot.Capture(options.Bounds, trigger, schedule.RequestId);
            token.ThrowIfCancellationRequested();
            await client.Upload(pair, frame, token);
            token.ThrowIfCancellationRequested(); schedule.Accepted(DateTimeOffset.UtcNow);
            Status?.Invoke($"Monitoring active — last capture {DateTime.Now:T} ({trigger})");
        }
        catch (RelayException exception) when (exception.Revoked)
        { await Pause(true); Status?.Invoke("Session ended — pair again from iOS"); }
        catch (RelayException exception)
        {
            schedule.Backoff(DateTimeOffset.UtcNow); nextPoll = DateTimeOffset.MinValue;
            Status?.Invoke(exception.Status == 409 ? "Monitoring active — waiting for current analysis" : "Monitoring active — relay temporarily unavailable");
        }
        catch (OperationCanceledException) { }
        catch (Exception exception) when (exception is HttpRequestException or InvalidDataException or System.Text.Json.JsonException)
        { schedule.Backoff(DateTimeOffset.UtcNow); Status?.Invoke("Monitoring active — connection unavailable; retrying"); }
        catch (Exception exception) when (exception is System.ComponentModel.Win32Exception or System.Runtime.InteropServices.ExternalException or InvalidOperationException)
        { await Pause(); Status?.Invoke("Capture unavailable — unlock Windows, check monitor, then resume"); }
        finally { busy = false; }
    }
}