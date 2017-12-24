# Run after all dependencies have been installed and environment set up
set -ex

travis-cargo build
travis-cargo test
travis-cargo bench
travis-cargo --only stable doc
