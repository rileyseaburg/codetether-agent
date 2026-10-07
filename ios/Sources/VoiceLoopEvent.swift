import Foundation

/// Events flowing from the mic engine into the conversation loop:
/// phase hints (energy/level changes) and turn-detection events.
struct VoiceLoopEvent {
    var phase: VoicePhase?
    var event: VADTurnDetector.Event?
    var probability: Float
    init(phase: VoicePhase? = nil, event: VADTurnDetector.Event? = nil, probability: Float = 0) {
        self.phase = phase
        self.event = event
        self.probability = probability
    }
}
