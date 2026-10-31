import Foundation

/// Prepare one chunk ahead while playing the current chunk, with bounded memory.
/// Structured children are cancelled when playback fails or Stop is pressed.
@MainActor
enum SpeechReadback {
    static func run(_ chunks: [String],
                    synthesize: @escaping @MainActor (String) async throws -> Data,
                    play: @escaping @MainActor (Data) async throws -> Void) async throws {
        guard let first = chunks.first else { return }
        try Task.checkCancellation()
        var audio = try await synthesize(first)
        for index in chunks.indices {
            try Task.checkCancellation()
            async let next = prepare(chunks, index: index + 1, synthesize: synthesize)
            try await play(audio)
            if let prepared = try await next { audio = prepared }
        }
    }
    private static func prepare(_ chunks: [String], index: Int,
                                synthesize: @MainActor (String) async throws -> Data) async throws -> Data? {
        guard index < chunks.count else { return nil }
        try Task.checkCancellation()
        return try await synthesize(chunks[index])
    }
}