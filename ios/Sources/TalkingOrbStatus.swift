import SwiftUI

/// The orb's icon and its failure overlay: a red badge showing the
/// failure message below the circle.
extension TalkingOrb {
    var icon: some View {
        Image(systemName: iconName)
            .font(.system(size: size * 0.36, weight: .medium))
            .foregroundStyle(.white)
            .rotationEffect(.degrees(spin && phase == .thinking ? 360 : 0))
            .animation(spin && phase == .thinking ? linear.repeatForever(autoreverses: false) : .default, value: spin)
    }

    var progressText: some View {
        Group {
            if case .failed(let message) = phase {
                VStack(spacing: 4) {
                    Image(systemName: "exclamationmark.triangle")
                    Text(message).font(.caption2).multilineTextAlignment(.center)
                }
                .foregroundStyle(.white)
                .frame(maxWidth: size * 1.6)
                .padding(8)
                .background(.red.opacity(0.8), in: RoundedRectangle(cornerRadius: 10))
                .offset(y: size * 0.9)
            }
        }
    }
}
