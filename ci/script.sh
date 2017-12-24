# Run after all dependencies have been installed and environment set up
set -ex

cargo build
cargo test
cargo bench
cargo doc
