# Emulates to `Verify` workflow found in GitHub.
verify:
    cargo fmt --check
    cargo clippy --no-deps --all-features -- -Dwarnings
    cargo test snapper --all-features --lib --no-fail-fast --verbose
    cargo test snapper --all-features --doc --no-fail-fast --verbose
    cargo build -p snapper --all-features --lib --release --verbose
    cargo build -p snapper --all-features --lib --verbose
