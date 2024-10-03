# Emulates to `Verify` workflow found in GitHub.
verify:
    cargo fmt --check
    cargo clippy --no-deps --all-features -- -Dwarnings
    cargo test snapr --all-features --lib --no-fail-fast --verbose
    cargo test snapr --all-features --doc --no-fail-fast --verbose
    cargo build -p snapr --all-features --lib --release --verbose
    cargo build -p snapr --all-features --lib --verbose
