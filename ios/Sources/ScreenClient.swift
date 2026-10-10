import Foundation

/// Ephemeral REST/SSE client. Each stream owns a session so cancellation closes its socket.
@MainActor
final class ScreenClient: ScreenNetworking {
    let configuration: URLSessionConfiguration
    let rest: ServerClient

    init(configuration: URLSessionConfiguration = .ephemeral) {
        // REST sets shorter deadlines; never share its mutable configuration with SSE.
        let secured = ScreenHTTP.secure(configuration.copy() as! URLSessionConfiguration)
        self.configuration = secured
        rest = ServerClient(configuration: secured.copy() as! URLSessionConfiguration)
    }
    func makeSession() -> URLSession {
        URLSession(configuration: configuration, delegate: RejectRedirects(), delegateQueue: nil)
    }
    func create(_ body: ScreenCreateBody, token: String) async throws -> ScreenSession {
        guard body.valid else { throw ScreenFailure.invalidInput }
        return try await rest.post("/companion/sessions", token: token, body: body)
    }
    func stop(_ id: UUID, token: String) async throws {
        let session = makeSession()
        defer { session.invalidateAndCancel() }
        let request = try ScreenHTTP.request("/companion/sessions/\(id.uuidString.lowercased())",
                                             token: token, method: "DELETE")
        let (data, response) = try await session.data(for: request)
        if let http = response as? HTTPURLResponse, [404, 410].contains(http.statusCode) { return }
        try ScreenHTTP.validate(response)
        let receipt = try JSONDecoder().decode(ScreenStopReceipt.self, from: data)
        guard receipt.stopped else { throw ScreenFailure.invalidResponse }
    }
}