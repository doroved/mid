import Foundation
import Mid // Ensure Mid.xcframework is added to your target's Frameworks

class MidManager {
    /// Get device hash (Machine ID)
    /// - Parameter serviceName: Unique service name for Keychain (e.g., "com.myapp.mid").
    ///   It is important to use a unique name to avoid collisions with other applications.
    /// - Returns: Hash string (SHA-256) or nil in case of error.
    static func get(serviceName: String) -> String? {
        // Convert Swift String to C String (pointer)
        guard let cServiceName = serviceName.cString(using: .utf8) else { return nil }

        // Call Rust function (ffi)
        // It returns a pointer to a string allocated in Rust heap
        // The mid_get function is exported from the Mid library
        guard let resultPtr = mid_get(cServiceName) else { return nil }

        // Convert C String back to Swift String
        let mid = String(cString: resultPtr)

        // IMPORTANT: Free memory allocated by Rust to avoid leaks
        mid_free_string(resultPtr)

        return mid
    }
}
