import SwiftUI

struct ScreenSetupView: View {
    @ObservedObject var model: ScreenModel
    @ObservedObject var chat: ChatModel

    var body: some View {
        Section("What should AI look for?") {
            if model.locked {
                Label(model.activeModel, systemImage: "cpu").font(.footnote)
            } else {
                ModelSelectorRow(chat: chat)
            }
            TextEditor(text: $model.prompt)
                .frame(minHeight: 90).disabled(model.locked)
                .accessibilityLabel("Screen analysis instructions")
            Picker("Capture interval", selection: $model.interval) {
                ForEach([15, 30, 60, 120, 300], id: \.self) { seconds in
                    Text("\(seconds) seconds").tag(seconds)
                }
            }.disabled(model.locked)
            if model.session == nil {
                Button(model.creating ? "Creating…" : "Pair Windows device") {
                    Task { await model.start(model: chat.selectedModel) }
                }
                .disabled(model.locked || chat.selectedModel.isEmpty || model.prompt.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty || model.prompt.count > 2000)
                .accessibilityIdentifier("screen-create")
            }
            Text("Analysis only: no remote control or tool execution. One session lasts up to one hour or 120 screenshots. Provider usage limits and data policies apply.")
                .font(.footnote).foregroundStyle(.secondary)
        }
    }
}