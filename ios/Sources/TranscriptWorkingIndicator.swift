import SwiftUI

/// Reserve one stable footer height so work-state changes cannot shift the transcript.
struct TranscriptWorkingIndicator: View {
    let busy: Bool
    var body: some View {
        HStack(spacing: 8) {
            if busy {
                ProgressView().controlSize(.small)
                Text("Working…").font(.footnote).foregroundStyle(.secondary)
            }
        }
        .frame(maxWidth: .infinity, minHeight: 40, maxHeight: 40)
        .accessibilityIdentifier("chat-thinking")
        .accessibilityHidden(!busy)
        .transaction { $0.animation = nil }
    }
}
