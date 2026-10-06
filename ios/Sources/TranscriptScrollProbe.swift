import SwiftUI
import UIKit

/// Attaches only to the enclosing transcript's scroll view; never replaces its delegate.
struct TranscriptScrollProbe: UIViewRepresentable {
    let controller: TranscriptScrollController
    func makeUIView(context: Context) -> TranscriptProbeView { TranscriptProbeView() }
    func updateUIView(_ view: TranscriptProbeView, context: Context) {
        view.controller = controller
        view.connect()
    }
    static func dismantleUIView(_ view: TranscriptProbeView, coordinator: ()) {
        view.controller?.disconnect()
    }
}

final class TranscriptProbeView: UIView {
    weak var controller: TranscriptScrollController?
    override func didMoveToWindow() { super.didMoveToWindow(); connect() }
    override func layoutSubviews() { super.layoutSubviews(); connect() }
    func connect() {
        var ancestor = superview
        while let view = ancestor {
            if let scrollView = view as? UIScrollView {
                controller?.connect(scrollView)
                return
            }
            ancestor = view.superview
        }
    }
}