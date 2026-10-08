namespace CodeTether.Companion;

/// <summary>Coalesces click bursts and prioritizes fresh owner requests over periodic capture.</summary>
internal sealed class CaptureSchedule
{
    private string? queued;
    private DateTimeOffset last = DateTimeOffset.MinValue;
    private DateTimeOffset retry = DateTimeOffset.MinValue;
    public string? RequestId { get; set; }
    public void Queue(string trigger) => queued = trigger;
    public string? Due(DateTimeOffset now, bool periodic, int interval)
    {
        if (now < retry) return null;
        if (RequestId is not null) return "manual";
        if (now - last < TimeSpan.FromSeconds(5)) return null;
        if (queued is not null) return queued;
        return periodic && now - last >= TimeSpan.FromSeconds(interval) ? "periodic" : null;
    }
    public void Accepted(DateTimeOffset now)
    { last = now; queued = null; RequestId = null; }
    public void Backoff(DateTimeOffset now) => retry = now.AddSeconds(5);
    public void Reset()
    { queued = null; RequestId = null; last = DateTimeOffset.MinValue; retry = DateTimeOffset.MinValue; }
}

/// <summary>Locally selected monitor and trigger preferences; no server can widen capture scope.</summary>
internal sealed record CaptureOptions(Rectangle Bounds, int Interval, bool Periodic, bool RightClick, bool DoubleClick);