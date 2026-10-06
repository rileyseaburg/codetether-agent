import Foundation

@MainActor
final class AgentSocket: AgentTransport {
    private var socket: URLSessionWebSocketTask?
    private let decoder = AgentFrameDecoder()
    func prompt(sessionID: String, message: String, status: @escaping (AgentFrame.Event) -> Void) async throws -> AgentReply {
        guard let token = try TokenStore.read() else { throw ClientError.missingToken }
        guard UUID(uuidString: sessionID) != nil else { throw ClientError.invalidResponse }
        let url = URL(string: "wss://server.codetether.run/api/realtime/session/\(sessionID)")!
        var request = URLRequest(url: url)
        request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        let session = URLSession(configuration: .ephemeral, delegate: RejectRedirects(), delegateQueue: nil)
        let connection = session.webSocketTask(with: request)
        socket = connection
        connection.maximumMessageSize = 16 * 1024 * 1024
        connection.resume()
        defer { connection.cancel(with: .normalClosure, reason: nil); session.invalidateAndCancel(); socket = nil }
        return try await withTaskCancellationHandler {
            let payload = try JSONEncoder().encode(Command(type: "prompt", message: message))
            try await connection.send(.string(String(decoding: payload, as: UTF8.self)))
            while true {
                let incoming = try await connection.receive()
                let data: Data
                switch incoming {
                case .data(let value): data = value
                case .string(let value): data = Data(value.utf8)
                @unknown default: throw ClientError.invalidResponse
                }
                let frame = try await decoder.decode(data)
                if let result = frame.result { return result }
                if frame.type == "error" { throw ClientError.agent(frame.message ?? "Agent request failed") }
                if let event = frame.event, event.payload != nil { status(event) }
            }
        } onCancel: {
            Task { try? await connection.send(.string("{\"type\":\"cancel\"}")); connection.cancel(with: .goingAway, reason: nil) }
        }
    }
    private struct Command: Encodable { let type: String; let message: String }
}
