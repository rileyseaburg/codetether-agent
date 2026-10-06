import SwiftUI

/// Parse markdown only when a completed message changes, not on audio/status updates.
struct StableMessageText: View, Equatable {
    let text: String
    var body: some View {
        Text(.init(text)).textSelection(.enabled)
    }
}
