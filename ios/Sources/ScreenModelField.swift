import SwiftUI

struct ScreenModelField: View {
    @ObservedObject var selection: ScreenModelSelection
    @State private var choosing = false

    var body: some View {
        TextField("provider/vision-model", text: $selection.selected)
            .textInputAutocapitalization(.never).autocorrectionDisabled()
            .accessibilityLabel("Screen vision model")
            .accessibilityIdentifier("screen-model")
        Button("Browse server models") { choosing = true }
            .sheet(isPresented: $choosing) { ScreenModelPicker(selection: selection) }
        Text("Select an image-capable model. This choice does not change Chat or Voice.")
            .font(.caption).foregroundStyle(.secondary)
    }
}
