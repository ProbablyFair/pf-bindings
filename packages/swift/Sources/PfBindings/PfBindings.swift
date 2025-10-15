import Foundation

/// Swift bindings for Probably Fair operations using the C FFI API
public struct PfBindings {
    
    /// Error type for Probably Fair bindings
    public enum Error: Swift.Error, LocalizedError {
        case verificationFailed(String)
        case registrationFailed(String)
        case internalError(String)
        
        public var errorDescription: String? {
            switch self {
            case .verificationFailed(let message):
                return "Verification failed: \(message)"
            case .registrationFailed(let message):
                return "Registration failed: \(message)"
            case .internalError(let message):
                return "Internal error: \(message)"
            }
        }
    }
    
    /// Verifies a bet receipt against its transcript
    /// - Parameters:
    ///   - receiptJson: The bet receipt as a JSON string
    ///   - transcriptJson: The transcript as a JSON string
    /// - Throws: PfError if verification fails
    public static func verifyBet(receiptJson: String, transcriptJson: String) throws {
        var error = PfError()
        let result = receiptJson.withCString { receiptPtr in
            transcriptJson.withCString { transcriptPtr in
                pf_verify_bet(receiptPtr, transcriptPtr, &error)
            }
        }
        
        switch result {
        case .success:
            // Clean up any error messages
            pf_free_error(error)
        case .verificationFailed:
            let message = errorMessage(from: error)
            pf_free_error(error)
            throw Error.verificationFailed(message)
        case .registrationFailed:
            let message = errorMessage(from: error)
            pf_free_error(error)
            throw Error.registrationFailed(message)
        case .invalidInput:
            let message = errorMessage(from: error)
            pf_free_error(error)
            throw Error.internalError("Invalid input: \(message)")
        case .internalError:
            let message = errorMessage(from: error)
            pf_free_error(error)
            throw Error.internalError(message)
        default:
            let message = errorMessage(from: error)
            pf_free_error(error)
            throw Error.internalError("Unknown error: \(message)")
        }
    }
    
    /// Registers a GDP package for use in betting operations
    /// - Parameter data: The GDP package binary data
    /// - Throws: PfError if registration fails
    public static func registerGdpPackage(_ data: Data) throws {
        try data.withUnsafeBytes { bytes in
            var error = PfError()
            let result = bytes.withMemoryRebound(to: UInt8.self) { ptr in
                pf_register_gdp_package(ptr.baseAddress, data.count, &error)
            }
            
            switch result {
            case .success:
                // Clean up any error messages
                pf_free_error(error)
            case .verificationFailed:
                let message = errorMessage(from: error)
                pf_free_error(error)
                throw Error.verificationFailed(message)
            case .registrationFailed:
                let message = errorMessage(from: error)
                pf_free_error(error)
                throw Error.registrationFailed(message)
            case .invalidInput:
                let message = errorMessage(from: error)
                pf_free_error(error)
                throw Error.internalError("Invalid input: \(message)")
            case .internalError:
                let message = errorMessage(from: error)
                pf_free_error(error)
                throw Error.internalError(message)
            default:
                let message = errorMessage(from: error)
                pf_free_error(error)
                throw Error.internalError("Unknown error: \(message)")
            }
        }
    }
    
    /// Get the library version
    public static func getVersion() -> String {
        return String(cString: pf_library_version())
    }
}

// Private helper functions
private extension PfBindings {
    static func errorMessage(from error: PfError) -> String {
        guard let messagePtr = error.message else { return "Unknown error" }
        let message = String(cString: messagePtr)
        return message
    }
}
