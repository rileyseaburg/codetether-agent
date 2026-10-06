import SwiftUI
import MediaPlayer

struct OutputVolume: UIViewRepresentable {
    func makeUIView(context: Context) -> MPVolumeView {
        let view = MPVolumeView()
        view.showsRouteButton = false
        view.accessibilityIdentifier = "output-volume"
        return view
    }
    func updateUIView(_ uiView: MPVolumeView, context: Context) {}
}
