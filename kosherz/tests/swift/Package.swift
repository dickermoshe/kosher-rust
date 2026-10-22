// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "KosherzSmoke",
    platforms: [.macOS("11.0")],
    dependencies: [
        .package(path: "../../generated/swift"),
    ],
    targets: [
        .executableTarget(
            name: "KosherzSmoke",
            dependencies: [.product(name: "Kosherz", package: "swift")]
        ),
    ]
)
