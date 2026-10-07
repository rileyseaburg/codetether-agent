import XCTest
@testable import CodeTether

@MainActor
final class PDFAttachmentTests: XCTestCase {
    func testPrepareKeepsPDFDataIntact() throws {
        let pdf = Data("%PDF-1.4\n%codetether\n".utf8)
        let prepared = try UserImage.prepare(pdf)
        XCTAssertEqual(prepared.kind, .document)
        XCTAssertEqual(prepared.data, pdf)
    }

    func testOversizePDFIsRejected() throws {
        var pdf = Data("%PDF-1.7\n".utf8)
        pdf.append(Data(repeating: 0x25, count: UserImage.maximumDocumentBytes))
        XCTAssertThrowsError(try UserImage.prepare(pdf))
    }

    func testAttachDocumentStoresAndRejectsWhenBusy() throws {
        let chat = ChatModel()
        try chat.attachDocument(Data("%PDF-1.4\n".utf8))
        XCTAssertEqual(chat.attachments.first?.kind, .document)
        chat.busy = true
        try chat.attachDocument(Data("%PDF-1.4\n".utf8))
        XCTAssertEqual(chat.attachments.count, 1)
    }
}
