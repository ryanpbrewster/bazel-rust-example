workspace(name = "bazel-rust-example")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive", "http_file")

# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    sha256 = "9d04e658878d23f4b00163a72da3db03ddb451273eb347df7d7c50838d698f49",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.26.0/rules_rust-v0.26.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

# Register a specific version just for the sake of stability.
rust_register_toolchains(
    edition = "2021",
    versions = ["1.66.1"],
    # Note: had to install a few packages to make cross-compilation work
    #   - gcc-aarch64-linux-gnu
    #   - g++-aarch64-linux-gnu
    extra_target_triples = ["aarch64-unknown-linux-gnu"],
)


load("@rules_rust//proto/prost:repositories.bzl", "rust_prost_dependencies")

rust_prost_dependencies()

load("@rules_rust//proto/prost:transitive_repositories.bzl", "rust_prost_transitive_repositories")

rust_prost_transitive_repositories()

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//build/rust:Cargo.lock",
    lockfile = "//build/rust:Cargo.Bazel.lock",
    annotations = {
        "protoc-gen-prost": [crate.annotation(
            gen_binaries = ["protoc-gen-prost"],
        )],
        "protoc-gen-tonic": [crate.annotation(
            gen_binaries = ["protoc-gen-tonic"],
        )],
    },
    packages = {
        "anyhow": crate.spec(
            version = "1.0.72",
        ),
        "clap": crate.spec(
            version = "4.3.19",
            features = ["derive"],
        ),
        "prost": crate.spec(
            version = "0.11.9",
        ),
        "prost-types": crate.spec(
            version = "0.11.9",
        ),
        "protoc-gen-prost": crate.spec(
            version = "0.2.3",
        ),
        "protoc-gen-tonic": crate.spec(
            version = "0.3.0",
        ),
        "tokio": crate.spec(
            version = "1.29.1",
            features = ["full"],
        ),
        "tonic": crate.spec(
            version = "0.9.2",
        ),
    },
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

register_toolchains("//build/rust:prost_toolchain")
