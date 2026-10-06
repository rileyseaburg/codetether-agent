import Foundation

extension AgentFrame.Event {
    private enum Keys: String, CodingKey { case kind, payload }
    /// Token deltas, reasoning and telemetry never enter published UI state.
    init(from decoder: Decoder) throws {
        let fields = try decoder.container(keyedBy: Keys.self)
        kind = try fields.decode(String.self, forKey: .kind)
        switch kind {
        case "item.started", "item.completed", "tool.started", "tool.completed", "tool.metadata":
            payload = try fields.decodeIfPresent(AgentFrame.Payload.self, forKey: .payload)
        default:
            payload = nil
        }
    }
}

actor AgentFrameDecoder {
    private let decoder = JSONDecoder()
    func decode(_ data: Data) throws -> AgentFrame {
        try autoreleasepool { try decoder.decode(AgentFrame.self, from: data) }
    }
}
