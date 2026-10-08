namespace CodeTether.Companion;

/// <summary>Short-lived device capability; never persisted or logged.</summary>
internal sealed record PairReceipt(string Id, string DeviceToken, int IntervalSeconds, DateTimeOffset ExpiresAt)
{
    public void Validate()
    {
        if (!Guid.TryParse(Id, out _) || DeviceToken.Length != 43 ||
            !DeviceToken.All(c => char.IsAsciiLetterOrDigit(c) || c is '_' or '-') ||
            IntervalSeconds is < 15 or > 300 || ExpiresAt <= DateTimeOffset.UtcNow ||
            ExpiresAt > DateTimeOffset.UtcNow.AddHours(1).AddMinutes(1))
            throw new InvalidDataException("Invalid pairing response.");
    }
}
internal sealed record PairBody(string Code);
internal sealed record DeviceCommand(string? RequestId);
internal sealed record FrameBody(string Image, string CapturedAt, string Trigger, string? RequestId);
internal sealed class RelayException(int status) : Exception("Relay request rejected.")
{
    public int Status { get; } = status;
    public bool Revoked => Status is 401 or 403 or 404 or 410;
}