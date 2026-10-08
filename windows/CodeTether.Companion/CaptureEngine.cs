namespace CodeTether.Companion;

/// <summary>Owns memory-only credentials and local capture permission, never hidden auto-start.</summary>
internal sealed partial class CaptureEngine : IDisposable
{
    private readonly RelayClient client = new();
    private readonly CaptureSchedule schedule = new();
    private CancellationTokenSource lifetime = new();
    private PairReceipt? pair;
    private CaptureOptions? options;
    private bool busy;
    private DateTimeOffset nextPoll;
    public bool Running { get; private set; }
    public bool Paired => pair is not null;
    public event Action<string>? Status;
    public async Task Pair(string code)
    {
        if (!System.Text.RegularExpressions.Regex.IsMatch(code, "^[A-Fa-f0-9]{12}$"))
            throw new InvalidDataException("Enter the twelve-character pairing code from iOS.");
        pair = await client.Pair(code.ToUpperInvariant(), lifetime.Token);
    }
    public void Start(CaptureOptions selected)
    {
        if (pair is null || pair.ExpiresAt <= DateTimeOffset.UtcNow) throw new InvalidOperationException("Pair with iOS first.");
        options = selected with { Interval = Math.Max(selected.Interval, pair.IntervalSeconds) };
        lifetime.Dispose(); lifetime = new CancellationTokenSource(); schedule.Reset();
        nextPoll = DateTimeOffset.MinValue; Running = true; Status?.Invoke("Monitoring active — selected monitor only");
    }
    public void Trigger(string trigger)
    {
        if (!Running || options is null) return;
        if (trigger == "manual" || trigger == "right_click" && options.RightClick || trigger == "double_click" && options.DoubleClick)
            schedule.Queue(trigger);
    }
    public async Task Pause(bool forget = false)
    {
        Running = false; lifetime.Cancel(); schedule.Reset();
        PairReceipt? previous = pair;
        if (forget) pair = null;
        Status?.Invoke(forget ? "Stopped — pairing forgotten" : "Paused — no screenshots captured");
        if (previous is null) return;
        try { await client.Pause(previous); }
        catch (Exception exception) when (exception is HttpRequestException or TaskCanceledException or RelayException)
        { Status?.Invoke("Local capture stopped; server confirmation unavailable"); }
    }
    public void Dispose()
    { Running = false; pair = null; lifetime.Cancel(); lifetime.Dispose(); client.Dispose(); }
}