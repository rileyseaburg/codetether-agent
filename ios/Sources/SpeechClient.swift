import Foundation

struct SpeechClient {
    private struct Request: Encodable { let script: String; let voice_id: String }
    func audio(text: String, voice: String) async throws -> Data {
        guard let token = try TokenStore.read() else { throw ClientError.missingToken }
        var request = URLRequest(url: ServerClient.origin.appendingPathComponent("tts/speak"))
        request.httpMethod = "POST"
        request.timeoutInterval = 180
        request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = try JSONEncoder().encode(Request(script: text, voice_id: voice))
        let config = URLSessionConfiguration.ephemeral
        config.urlCache = nil
        config.httpCookieStorage = nil
        let session = URLSession(configuration: config, delegate: RejectRedirects(), delegateQueue: nil)
        defer { session.invalidateAndCancel() }
        let (data, response) = try await session.data(for: request)
        guard let response = response as? HTTPURLResponse else { throw ClientError.invalidResponse }
        guard response.statusCode == 200 else { throw ClientError.http(response.statusCode) }
        guard response.mimeType == "audio/wav", data.count > 44,
              String(data: data.prefix(4), encoding: .ascii) == "RIFF" else {
            throw ClientError.invalidResponse
        }
        return data
    }
}
