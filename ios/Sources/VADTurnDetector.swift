import Foundation

/// Hysteresis turn detector over Silero speech probabilities.
/// Fires `.speechStart` after sustained probability ≥ startThreshold
/// (minSpeechMs), and `.speechEnd` after probability < endThreshold
/// for minSilenceMs — the moment we consider the user's turn over.
struct VADTurnDetector {
    enum Event: Equatable { case speechStart, speechEnd }

    var startThreshold: Float
    var endThreshold: Float
    var minSpeechMs: Int
    var minSilenceMs: Int
    private(set) var inSpeech = false
    private var speechRun = 0
    private var silenceRun = 0

    init(startThreshold: Float = 0.55, endThreshold: Float = 0.35,
         minSpeechMs: Int = 256, minSilenceMs: Int = 480) {
        self.startThreshold = startThreshold
        self.endThreshold = endThreshold
        self.minSpeechMs = minSpeechMs
        self.minSilenceMs = minSilenceMs
    }

    /// Feed one probability (one 256 ms frame); returns events in order.
    mutating func feed(_ probability: Float) -> [Event] {
        let stepMs = 256
        var events: [Event] = []
        if !inSpeech {
            if probability >= startThreshold {
                speechRun += stepMs
                if speechRun >= minSpeechMs { inSpeech = true; events.append(.speechStart) }
            } else {
                speechRun = 0
            }
            silenceRun = 0
        } else {
            if probability < endThreshold {
                silenceRun += stepMs
                if silenceRun >= minSilenceMs { inSpeech = false; events.append(.speechEnd); speechRun = 0; silenceRun = 0 }
            } else {
                silenceRun = 0
            }
        }
        return events
    }

    mutating func reset() {
        inSpeech = false; speechRun = 0; silenceRun = 0
    }
}
