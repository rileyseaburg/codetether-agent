import SwiftUI

struct ScreenSetupView: View {
    @ObservedObject var model: ScreenModel
    @ObservedObject var selection: ScreenModelSelection

    var body: some View {
        Section("What should AI look for?") {
            if model.locked {
                Label(model.activeModel, systemImage: "cpu").font(.footnote)
            } else {
                ScreenModelField(selection: selection)
            }
            TextEditor(text: $model.prompt)
                .frame(minHeight: 90).disabled(model.locked)
                .accessibilityLabel("Screen analysis instructions")
            Text("\(model.prompt.utf16.count)/2,000")
                .font(.caption).foregroundStyle(ScreenInput.text(model.prompt) ? Color.secondary : .red)
            Picker("Capture interval", selection: $model.interval) {
                ForEach([15, 30, 60, 120, 300], id: \.self) { seconds in
                    Text("\(seconds) seconds").tag(seconds)
                }
            }.disabled(model.locked)
            if model.session == nil {
                Button(model.creating ? "Creating…" : "Pair Windows device") {
                    Task { await model.start(model: selection.selected) }
                }
                .disabled(model.locked || !ScreenCreateBody(model: selection.selected,
                    prompt: model.prompt, interval_seconds: model.interval).valid)
                .accessibilityIdentifier("screen-create")
            }
            Text("Screen analysis and directly requested typing into the focused Windows field. No clicking, submission or tool execution. One session lasts up to one hour or 120 screenshots. Provider usage limits and data policies apply.")
                .font(.footnote).foregroundStyle(.secondary)
        }
    }
}
