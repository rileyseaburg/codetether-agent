import Foundation

/// Playback owner captured at turn start; switching tabs must not replay a voice reply.
enum ReplySpeechOwner: Equatable {
    case chat
    case voice
}