import Foundation

/// Incremental SSE framing: heartbeat comments are ignored, data fields join with LF.
struct ScreenSSEParser {
    private var data: [String] = []
    private var size = 0
    private var firstLine = true
    static let maximumEventBytes = 1_048_576

    mutating func consume(_ rawLine: String) throws -> ScreenEvent? {
        var line = rawLine
        if firstLine { firstLine = false; if line.first == "\u{FEFF}" { line.removeFirst() } }
        if line.last == "\r" { line.removeLast() }
        if line.isEmpty {
            guard !data.isEmpty else { return nil }
            defer { data.removeAll(keepingCapacity: true); size = 0 }
            guard let event = try? JSONDecoder().decode(ScreenEvent.self,
                from: Data(data.joined(separator: "\n").utf8)), event.seq >= 0 else {
                throw ScreenFailure.invalidEvent
            }
            return event
        }
        guard !line.hasPrefix(":"), line == "data" || line.hasPrefix("data:") else { return nil }
        var value = line == "data" ? "" : String(line.dropFirst(5))
        if value.first == " " { value.removeFirst() }
        size += value.utf8.count + 1
        guard size <= Self.maximumEventBytes else { throw ScreenFailure.invalidEvent }
        data.append(value)
        return nil
    }
}