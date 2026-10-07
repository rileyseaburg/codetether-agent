import SwiftUI

/// Full-screen voice mode: the talking orb center-stage, live transcript
/// and reply captions below, status/errors surfaced inline. Tap orb =
/// context action (start / send / interrupt / barge-in).
struct VoiceModeView: View {
    @ObservedObject var loop: VoiceLoop
    @ObservedObject var mic: VoiceMicEngine

    var body: some View {
        VStack(spacing: 24) {
            status
            Spacer()
            TalkingOrb(phase: loop.phase, probability: loop.probability, levelDb: mic.inputLevelDb)
                .onTapGesture { loop.tapped() }
            Spacer()
            captions
        }
        .padding(24)
        .buildFrame()
        .background(Color(.systemBackground).ignoresSafeArea())
        .accessibilityIdentifier("voice-mode")
    }
}

/// Frame + background modifiers for the voice screen (split for
/// file-size limits).
private extension View {
    func buildFrame() -> some View {
        frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}
