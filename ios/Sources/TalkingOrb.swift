import SwiftUI

/// The talking circle: idle dim, listening = breathing cyan reacting to
/// mic level, userSpeaking = bright ring pulses, thinking = indigo
/// rotation, speaking = green waves. Pure function of phase + level.
/// Styling lives in `TalkingOrbStyles`; error overlay in `TalkingOrbStatus`.
struct TalkingOrb: View {
    let phase: VoicePhase
    let probability: Float
    let levelDb: Float
    // Internal: the styles/status extensions live in separate files.
    @State var breathe = false
    @State var spin = false

    var body: some View {
        ZStack {
            Circle()
                .fill(color.opacity(0.14))
                .frame(width: size * 1.5, height: size * 1.5)
                .scaleEffect(breathe ? 1.06 : 0.94)
                .animation(ease.repeatForever(autoreverses: true), value: breathe)
            Circle()
                .stroke(color.opacity(0.5), lineWidth: 2)
                .frame(width: size * 1.2, height: size * 1.2)
                .scaleEffect(scale)
                .opacity(0.35)
            Circle()
                .fill(color.opacity(0.85))
                .frame(width: size, height: size)
                .scaleEffect(scale)
            icon.overlay(progressText)
        }
        .frame(maxWidth: .infinity)
        .contentShape(Circle().size(width: size * 1.5, height: size * 1.5))
        .onAppear { breathe = true; spin = true }
        .accessibilityElement(children: .ignore)
        .accessibilityLabel(accessibilityText)
        .accessibilityIdentifier("talking-orb")
    }
}
