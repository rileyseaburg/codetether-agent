import Foundation

/// Constructs fixed-origin requests and checks status without exposing response bodies.
enum ScreenHTTP {
    static func request(_ path: String, token: String, method: String = "GET") throws -> URLRequest {
        let clean = token.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !clean.isEmpty, clean.rangeOfCharacter(from: .controlCharacters) == nil else {
            throw ClientError.missingToken
        }
        var request = URLRequest(url: try ServerURL.path(path))
        request.httpMethod = method
        request.cachePolicy = .reloadIgnoringLocalCacheData
        request.setValue("Bearer \(clean)", forHTTPHeaderField: "Authorization")
        request.setValue("no-store", forHTTPHeaderField: "Cache-Control")
        request.setValue("application/json", forHTTPHeaderField: "Accept")
        return request
    }

    static func validate(_ response: URLResponse, stream: Bool = false) throws {
        guard let http = response as? HTTPURLResponse else { throw ScreenFailure.invalidResponse }
        guard http.statusCode == 200 else { throw ScreenFailure.http(http.statusCode) }
        if stream && http.mimeType?.lowercased() != "text/event-stream" {
            throw ScreenFailure.invalidResponse
        }
    }

    static func secure(_ config: URLSessionConfiguration) -> URLSessionConfiguration {
        config.urlCache = nil; config.httpCookieStorage = nil; config.httpShouldSetCookies = false
        config.requestCachePolicy = .reloadIgnoringLocalCacheData
        config.timeoutIntervalForRequest = 60
        config.timeoutIntervalForResource = 3900
        return config
    }
}