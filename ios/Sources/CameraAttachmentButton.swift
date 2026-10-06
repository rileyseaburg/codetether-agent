import SwiftUI

struct CameraAttachmentButton: View {
    @ObservedObject var chat: ChatModel
    @Environment(\.openURL) private var openURL
    @State private var showingCamera = false
    @State private var checking = false
    @State private var access = CameraAccess.ready
    @State private var showingAlert = false

    var body: some View {
        Button {
            checking = true
            Task { @MainActor in
                access = await CameraAccess.request()
                checking = false
                guard !chat.busy, chat.attachments.count < UserImage.maximumAttachments else { return }
                if access == .ready { showingCamera = true }
                else { showingAlert = true }
            }
        } label: {
            Label("Camera", systemImage: "camera")
        }
        .accessibilityIdentifier("capture-image")
        .disabled(checking || chat.busy || chat.attachments.count >= UserImage.maximumAttachments)
        .fullScreenCover(isPresented: $showingCamera) {
            CameraPicker { image in
                showingCamera = false
                chat.attachCapture(image)
            }.ignoresSafeArea()
        }
        .alert("Camera access", isPresented: $showingAlert) {
            if access == .denied {
                Button("Open Settings") {
                    if let url = URL(string: UIApplication.openSettingsURLString) { openURL(url) }
                }
            }
            Button("Cancel", role: .cancel) {}
        } message: { Text(access.message) }
    }
}