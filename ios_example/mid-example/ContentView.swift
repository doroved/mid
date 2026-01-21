import SwiftUI
import UIKit

struct ContentView: View {
    @State private var machineID: String = "Generating..."

    // Use a unique identifier for your application
    let serviceName = "io.github.doroved.mid.example"

    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "cpu")
                .imageScale(.large)
                .foregroundStyle(.tint)

            Text("Machine ID:")
                .font(.headline)

            ScrollView(.horizontal, showsIndicators: false) {
                Text(machineID)
                    .font(.system(.body, design: .monospaced))
                    .padding()
            }
            .background(Color.gray.opacity(0.1))
            .cornerRadius(8)
            .onTapGesture {
                UIPasteboard.general.string = machineID
                let generator = UIImpactFeedbackGenerator(style: .medium)
                generator.impactOccurred()
            }

            Text("Tap on ID to copy")
                .font(.caption)
                .foregroundColor(.secondary)

            Button("Regenerate (Test)") {
                // In a real application, the ID will not change as long as the record exists in the Keychain,
                // even if you call get() again.
                loadMID()
            }
        }
        .padding()
        .onAppear {
            loadMID()
        }
    }

    func loadMID() {
        if let mid = MidManager.get(serviceName: serviceName) {
            self.machineID = mid
        } else {
            self.machineID = "Error retrieving ID"
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
