import SwiftUI

/// Copy the original message, not the shortened on-screen preview.
struct MessageCopyMenu: View {
    let text: String
    @State private var copied = false
    @State private var copyFailed = false

    var body: some View {
        Menu {
            ForEach(MessageCopyFormat.allCases) { format in
                Button(format.rawValue) { copy(format) }
                    .accessibilityIdentifier("copy-as-\(format.id)")
            }
        } label: {
            Label(copied ? "Copied" : "Copy", systemImage: copied ? "checkmark" : "doc.on.doc")
                .font(.caption)
        }
        .accessibilityIdentifier("message-copy")
        .disabled(text.isEmpty)
        .alert("Could not copy rich text", isPresented: $copyFailed) {
            Button("OK", role: .cancel) { }
        } message: { Text("Try Plain text or Markdown instead.") }
    }

    private func copy(_ format: MessageCopyFormat) {
        do {
            let payload = try MessageCopyPayload.make(text, format: format)
            UIPasteboard.general.setItems([payload])
            copied = true
            UIAccessibility.post(notification: .announcement, argument: "Copied as \(format.rawValue)")
        } catch { copyFailed = true }
    }
}
