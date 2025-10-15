// swift-tools-version:5.7
import PackageDescription

let package = Package(
    name: "PfBindings",
    platforms: [
        .macOS(.v10_15),
        .iOS(.v13),
        .tvOS(.v13),
        .watchOS(.v6),
    ],
    products: [
        .library(
            name: "PfBindings",
            targets: ["PfBindings"]),
    ],
    dependencies: [],
    targets: [
        .target(
            name: "PfBindings",
            dependencies: [],
            path: ".",
            sources: ["Sources"],
            cSettings: [
                .headerSearchPath("include"),
                .define("PF_BINDINGS_STATIC", .when(platforms: [.iOS, .macOS]))
            ],
            linkerSettings: [
                .linkedFramework("Security"),
                .linkedLibrary("c")
            ])
    ]
)
