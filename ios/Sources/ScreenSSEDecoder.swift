import Foundation

/// Byte framing preserves blank separators, fragmented UTF-8, and CR/LF variants.
struct ScreenSSEDecoder {
    private var line: [UInt8] = []
    private var afterCR = false
    private var parser = ScreenSSEParser()

    mutating func consume(_ byte: UInt8) throws -> ScreenEvent? {
        if afterCR {
            afterCR = false
            if byte == 10 { return nil }
        }
        if byte == 10 || byte == 13 {
            afterCR = byte == 13
            guard let text = String(bytes: line, encoding: .utf8) else {
                throw ScreenFailure.invalidEvent
            }
            line.removeAll(keepingCapacity: true)
            return try parser.consume(text)
        }
        guard line.count < ScreenSSEParser.maximumEventBytes else { throw ScreenFailure.invalidEvent }
        line.append(byte)
        return nil
    }
}