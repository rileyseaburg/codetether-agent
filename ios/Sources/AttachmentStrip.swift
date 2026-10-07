import SwiftUI

/// Horizontally scrolling row of pending attachment thumbnails.
struct AttachmentStrip: View {
    @ObservedObject var chat: ChatModel
    var body: some View {
        ScrollView(.horizontal) {
            HStack {
                ForEach(chat.attachments) { attachment in
                    if attachment.kind == .document {
                        DocumentThumbnail { chat.attachments.removeAll { $0.id == attachment.id } }
                    } else if let preview = UIImage(data: attachment.data) {
                        ImagePreview(image: preview) { chat.attachments.removeAll { $0.id == attachment.id } }
                    }
                }
            }
        }
    }
}
