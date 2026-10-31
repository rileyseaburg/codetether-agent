import Foundation

extension ReadbackTransport {
    func read(_ text: String) async throws {
        try await read(chunks: SpeechChunks.split(text)) { chunk in
            try await SpeechClient().audio(text: chunk, voice: "af_heart")
        }
    }
    func read(chunks: [String], synthesize: @escaping @MainActor (String) async throws -> Data) async throws {
        try Task.checkCancellation()
        begin()
        let current = epoch
        try await withTaskCancellationHandler {
            try await withCheckedThrowingContinuation { continuation in
                completion = continuation
                producer = Task {
                    do {
                        for chunk in chunks {
                            let audio = try await synthesize(chunk)
                            try Task.checkCancellation()
                            guard current == epoch else { return }
                            timeline.durations.append(try cache.append(audio))
                            duration = timeline.duration
                            if player == nil { try playCurrent() }
                            refresh()
                        }
                        generating = false
                        if player == nil { finish() }
                    } catch { if current == epoch { finish(error) } }
                }
            }
        } onCancel: { Task { @MainActor in if self.epoch == current { self.stop() } } }
    }
}