import Dispatch
import Foundation
import ImageIO

let VK_OK: Int32 = 0
let VK_INVALID_ARGUMENT: Int32 = -1
let VK_UNAVAILABLE_ON_THIS_MACOS: Int32 = -2
let VK_TIMED_OUT: Int32 = -3
let VK_UNAVAILABLE_ON_THIS_PLATFORM: Int32 = -4
let VK_ANALYZER_NOT_SUPPORTED: Int32 = -10
let VK_FRAMEWORK_ERROR: Int32 = -20
let VK_UNKNOWN: Int32 = -99

@_cdecl("vk_string_free")
public func vk_string_free(_ string: UnsafeMutablePointer<CChar>?) {
    guard let string else { return }
    free(string)
}

@inline(__always)
func vkCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
func vkRetain(_ object: NSObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func vkBorrow<T: NSObject>(
    _ ptr: UnsafeMutableRawPointer,
    as type: T.Type = T.self
) -> T {
    let typed = UnsafeRawPointer(ptr).assumingMemoryBound(to: T.self)
    return Unmanaged<T>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func vkRelease(_ ptr: UnsafeMutableRawPointer) {
    let typed = UnsafeRawPointer(ptr).assumingMemoryBound(to: NSObject.self)
    Unmanaged<NSObject>.fromOpaque(UnsafeRawPointer(typed)).release()
}

enum VKBridgeError: Error, CustomStringConvertible {
    case invalidArgument(String)
    case unavailableOnThisMacOS(String)
    case unavailableOnThisPlatform(String)
    case timedOut(String)
    case analyzerNotSupported(String)
    case framework(String)
    case unknown(String)

    var description: String {
        switch self {
        case let .invalidArgument(message),
            let .unavailableOnThisMacOS(message),
            let .unavailableOnThisPlatform(message),
            let .timedOut(message),
            let .analyzerNotSupported(message),
            let .framework(message),
            let .unknown(message):
            return message
        }
    }

    var statusCode: Int32 {
        switch self {
        case .invalidArgument:
            return VK_INVALID_ARGUMENT
        case .unavailableOnThisMacOS:
            return VK_UNAVAILABLE_ON_THIS_MACOS
        case .unavailableOnThisPlatform:
            return VK_UNAVAILABLE_ON_THIS_PLATFORM
        case .timedOut:
            return VK_TIMED_OUT
        case .analyzerNotSupported:
            return VK_ANALYZER_NOT_SUPPORTED
        case .framework:
            return VK_FRAMEWORK_ERROR
        case .unknown:
            return VK_UNKNOWN
        }
    }
}

func vkStatus(from error: Error) -> Int32 {
    if let bridgeError = error as? VKBridgeError {
        return bridgeError.statusCode
    }
    return VK_FRAMEWORK_ERROR
}

final class VKAsyncResultBox<T>: @unchecked Sendable {
    private let lock = NSLock()
    private var storedResult: Result<T, Error>?

    func store(_ result: Result<T, Error>) {
        lock.lock()
        storedResult = result
        lock.unlock()
    }

    func load() -> Result<T, Error>? {
        lock.lock()
        defer { lock.unlock() }
        return storedResult
    }
}

func vkWaitForSemaphore(
    _ semaphore: DispatchSemaphore,
    timeoutSeconds: TimeInterval,
    label: String
) throws {
    if Thread.isMainThread {
        let deadline = Date().addingTimeInterval(timeoutSeconds)
        while semaphore.wait(timeout: .now()) == .timedOut {
            if Date() >= deadline {
                throw VKBridgeError.timedOut(
                    "VisionKit \(label) timed out after \(Int(timeoutSeconds)) seconds"
                )
            }
            RunLoop.current.run(
                mode: .default,
                before: Date().addingTimeInterval(0.01)
            )
        }
    } else {
        let timeout = DispatchTime.now() + .milliseconds(Int(timeoutSeconds * 1_000))
        if semaphore.wait(timeout: timeout) == .timedOut {
            throw VKBridgeError.timedOut(
                "VisionKit \(label) timed out after \(Int(timeoutSeconds)) seconds"
            )
        }
    }
}

public func vk_block_on_async<T>(
    timeoutSeconds: TimeInterval = 60,
    work: @escaping () async throws -> T
) throws -> T {
    let semaphore = DispatchSemaphore(value: 0)
    let box = VKAsyncResultBox<T>()

    Task {
        do {
            box.store(.success(try await work()))
        } catch {
            box.store(.failure(error))
        }
        semaphore.signal()
    }

    try vkWaitForSemaphore(
        semaphore,
        timeoutSeconds: timeoutSeconds,
        label: "async call"
    )

    guard let result = box.load() else {
        throw VKBridgeError.unknown(
            "missing async result after semaphore completed"
        )
    }
    return try result.get()
}

public func vk_block_on_main_actor_async<T>(
    timeoutSeconds: TimeInterval = 60,
    work: @escaping @MainActor () async throws -> T
) throws -> T {
    let semaphore = DispatchSemaphore(value: 0)
    let box = VKAsyncResultBox<T>()

    Task { @MainActor in
        do {
            box.store(.success(try await work()))
        } catch {
            box.store(.failure(error))
        }
        semaphore.signal()
    }

    try vkWaitForSemaphore(
        semaphore,
        timeoutSeconds: timeoutSeconds,
        label: "main-actor async call"
    )

    guard let result = box.load() else {
        throw VKBridgeError.unknown(
            "missing main-actor async result after semaphore completed"
        )
    }
    return try result.get()
}

func vkOnMainActor<T>(
    timeoutSeconds: TimeInterval = 60,
    _ work: @escaping @MainActor () throws -> T
) throws -> T {
    if Thread.isMainThread {
        return try MainActor.assumeIsolated {
            try work()
        }
    }

    let semaphore = DispatchSemaphore(value: 0)
    let box = VKAsyncResultBox<T>()
    Task { @MainActor in
        do {
            box.store(.success(try work()))
        } catch {
            box.store(.failure(error))
        }
        semaphore.signal()
    }

    let timeout = DispatchTime.now() + .milliseconds(Int(timeoutSeconds * 1_000))
    if semaphore.wait(timeout: timeout) == .timedOut {
        throw VKBridgeError.timedOut(
            "VisionKit main-actor call timed out after \(Int(timeoutSeconds)) seconds"
        )
    }

    guard let result = box.load() else {
        throw VKBridgeError.unknown("missing main-actor result")
    }
    return try result.get()
}

func vkEncodeJSON<T: Encodable>(_ value: T) throws -> String {
    let data = try JSONEncoder().encode(value)
    guard let string = String(data: data, encoding: .utf8) else {
        throw VKBridgeError.unknown("failed to encode JSON as UTF-8")
    }
    return string
}

func vkDecodeJSON<T: Decodable>(
    _ cString: UnsafePointer<CChar>?,
    as type: T.Type
) throws -> T {
    guard let cString else {
        throw VKBridgeError.invalidArgument("missing JSON payload")
    }
    let data = Data(String(cString: cString).utf8)
    do {
        return try JSONDecoder().decode(T.self, from: data)
    } catch {
        throw VKBridgeError.invalidArgument(
            "invalid JSON payload: \(error.localizedDescription)"
        )
    }
}

func vkWriteJSON<T: Encodable>(
    _ value: T,
    to outPointer: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) throws {
    outPointer.pointee = vkCString(try vkEncodeJSON(value))
}

func vkRequireString(_ cString: UnsafePointer<CChar>?, field: String) throws -> String {
    guard let cString else {
        throw VKBridgeError.invalidArgument("missing \(field)")
    }
    return String(cString: cString)
}

func vkRequireFilePath(
    _ cString: UnsafePointer<CChar>?,
    field: String
) throws -> String {
    let path = try vkRequireString(cString, field: field)
    guard FileManager.default.fileExists(atPath: path) else {
        throw VKBridgeError.invalidArgument(
            "file does not exist at path: \(path)"
        )
    }
    return path
}

func vkImageOrientation(from raw: UInt32) throws -> CGImagePropertyOrientation {
    guard let orientation = CGImagePropertyOrientation(rawValue: raw) else {
        throw VKBridgeError.invalidArgument(
            "invalid image orientation raw value: \(raw)"
        )
    }
    return orientation
}

struct VKAreaSupportPayload: Codable {
    var area: String
    var currentPlatform: String
    var availableOnCurrentPlatform: Bool
    var availability: String
    var reason: String?
    var members: [String]
    var notes: [String]
}

func vkAreaSupportPayload(
    area: String,
    availableOnCurrentPlatform: Bool,
    availability: String,
    reason: String?,
    members: [String],
    notes: [String] = []
) -> VKAreaSupportPayload {
    VKAreaSupportPayload(
        area: area,
        currentPlatform: "macOS",
        availableOnCurrentPlatform: availableOnCurrentPlatform,
        availability: availability,
        reason: reason,
        members: members,
        notes: notes
    )
}
