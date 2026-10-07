import Foundation

/// Phases of the voice conversation loop, mirrored 1:1 onto the orb.
enum VoicePhase: Equatable {
    case idle
    case listening
    case userSpeaking
    case thinking
    case speaking
    case failed(String)
}
