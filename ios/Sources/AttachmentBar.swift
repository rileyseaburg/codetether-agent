import SwiftUI
import PhotosUI
import UniformTypeIdentifiers

struct AttachmentBar: View {
    @ObservedObject var chat: ChatModel
    @State private var selection: [PhotosPickerItem] = []
    @State private var showingDocumentPicker = false
    var body: some View {
        HStack {
            CameraAttachmentButton(chat: chat)
            PhotosPicker(selection: $selection,
                         maxSelectionCount: max(1, UserImage.maximumAttachments - chat.attachments.count),
                         matching: .images) {
                Label("Photos", systemImage: "photo.badge.plus")
            }.accessibilityIdentifier("add-image").disabled(chat.attachments.count >= UserImage.maximumAttachments)
            Button { showingDocumentPicker = true } label: {
                Label("Files", systemImage: "doc.badge.plus")
            }.accessibilityIdentifier("add-pdf").disabled(chat.attachments.count >= UserImage.maximumAttachments)
            AttachmentStrip(chat: chat)
        }.frame(height: chat.attachments.isEmpty ? 36 : 64).padding(.horizontal).disabled(chat.busy)
        .fileImporter(isPresented: $showingDocumentPicker, allowedContentTypes: [.pdf],
                      allowsMultipleSelection: true) { DocumentImport.handle($0, chat: chat) }
        .onChange(of: selection) { _, items in
            Task { await PhotoImport.handle(items, chat: chat); selection = [] }
        }
    }
}
