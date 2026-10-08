using System.Net.Http.Headers;
using System.Net.Http.Json;
using System.Text.Json;

namespace CodeTether.Companion;

/// <summary>Fixed HTTPS origin, bounded responses, no redirects or stored credentials.</summary>
internal sealed class RelayClient : IDisposable
{
    private readonly HttpClient http = new(new HttpClientHandler { AllowAutoRedirect = false })
    { BaseAddress = new Uri("https://server.codetether.run/companion/"), Timeout = TimeSpan.FromSeconds(20), MaxResponseContentBufferSize = 8192 };
    private static readonly JsonSerializerOptions Json = new() { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower, PropertyNameCaseInsensitive = false };
    public async Task<PairReceipt> Pair(string code, CancellationToken token)
    {
        using HttpResponseMessage response = await Send(HttpMethod.Post, "pair", new PairBody(code), null, token);
        PairReceipt receipt = await response.Content.ReadFromJsonAsync<PairReceipt>(Json, token) ?? throw new InvalidDataException();
        receipt.Validate(); return receipt;
    }
    public async Task<DeviceCommand> Poll(PairReceipt pair, CancellationToken token)
    {
        using HttpResponseMessage response = await Send(HttpMethod.Get, $"sessions/{pair.Id}/commands", null, pair.DeviceToken, token);
        DeviceCommand command = await response.Content.ReadFromJsonAsync<DeviceCommand>(Json, token) ?? throw new InvalidDataException();
        if (command.RequestId is not null && !Guid.TryParse(command.RequestId, out _)) throw new InvalidDataException();
        return command;
    }
    public async Task Upload(PairReceipt pair, FrameBody frame, CancellationToken token)
    { using HttpResponseMessage response = await Send(HttpMethod.Post, $"sessions/{pair.Id}/frames", frame, pair.DeviceToken, token); }
    public async Task Pause(PairReceipt pair)
    { using HttpResponseMessage response = await Send(HttpMethod.Post, $"sessions/{pair.Id}/pause", new { }, pair.DeviceToken, CancellationToken.None); }
    private async Task<HttpResponseMessage> Send(HttpMethod method, string path, object? body, string? bearer, CancellationToken token)
    {
        using HttpRequestMessage request = new(method, path);
        if (bearer is not null) request.Headers.Authorization = new AuthenticationHeaderValue("Bearer", bearer);
        request.Headers.CacheControl = new CacheControlHeaderValue { NoStore = true };
        if (body is not null) request.Content = JsonContent.Create(body, options: Json);
        HttpResponseMessage response = await http.SendAsync(request, token);
        if (response.IsSuccessStatusCode) return response;
        int status = (int)response.StatusCode; response.Dispose(); throw new RelayException(status);
    }
    public void Dispose() => http.Dispose();
}