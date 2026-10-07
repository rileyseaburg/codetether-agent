import SwiftUI

/// Compact row above the composer showing the active model; tap to change it.
struct ModelSelectorRow: View {
    @ObservedObject var chat: ChatModel
    @State private var showPicker = false
    var onPresent: () -> Void = {}
    var onDismiss: () -> Void = {}
    var body: some View {
        Button {
            onPresent()
            showPicker = true
        } label: {
            HStack {
                Image(systemName: "cpu")
                Text(chat.selectedModel.isEmpty ? "Choose model" : chat.selectedModel)
                    .font(.footnote).lineLimit(1).truncationMode(.middle)
                Image(systemName: "chevron.up.chevron.down").font(.caption2)
                Spacer()
            }
        }
        .buttonStyle(.plain).foregroundStyle(.secondary)
        .padding(.horizontal).padding(.vertical, 2)
        .disabled(chat.busy || chat.loading || chat.models.isEmpty)
        .accessibilityIdentifier("model-selector")
        .sheet(isPresented: $showPicker, onDismiss: onDismiss) { ModelPickerView(chat: chat) }
    }
}