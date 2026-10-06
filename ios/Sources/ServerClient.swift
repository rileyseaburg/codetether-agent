import Foundation

/// Fixed HTTPS origin; credentials never follow redirects or enter a cache.
final class ServerClient {
    static let origin = URL(string: "https://server.codetether.run")!
    private let session: URLSession

    init(configuration: URLSessionConfiguration = .ephemeral) {
        configuration.urlCache = nil
        configuration.httpCookieStorage = nil
        configuration.requestCachePolicy = .reloadIgnoringLocalCacheData
        configuration.timeoutIntervalForRequest = 30
        configuration.timeoutIntervalForResource = 180
        session = URLSession(configuration: configuration,
                             delegate: RejectRedirects(), delegateQueue: nil)
    }

    func get<T: Decodable>(_ path: String, token: String) async throws -> T {
        try await send(path, token: token, body: nil)
    }
    func post<B: Encodable, T: Decodable>(_ path: String, token: String, body: B) async throws -> T {
        try await send(path, token: token, body: JSONEncoder().encode(body))
    }
    private func send<T: Decodable>(_ path: String, token: String, body: Data?) async throws -> T {
        let clean = token.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !clean.isEmpty, clean.rangeOfCharacter(from: .controlCharacters) == nil else {
            throw ClientError.missingToken
        }
        var request = URLRequest(url: try ServerURL.path(path))
        request.httpMethod = body == nil ? "GET" : "POST"
        request.httpBody = body
        if body != nil { request.setValue("application/json", forHTTPHeaderField: "Content-Type") }
        request.timeoutInterval = body == nil ? 30 : 180
        request.setValue("Bearer \(clean)", forHTTPHeaderField: "Authorization")
        request.setValue("application/json", forHTTPHeaderField: "Accept")
        let (data, response) = try await session.data(for: request)
        guard let response = response as? HTTPURLResponse else { throw ClientError.invalidResponse }
        switch response.statusCode {
        case 200: break
        case 401: throw ClientError.unauthorized
        case 403: throw ClientError.forbidden
        default: throw ClientError.http(response.statusCode)
        }
        do { return try JSONDecoder().decode(T.self, from: data) }
        catch { throw ClientError.invalidResponse }
    }
}
