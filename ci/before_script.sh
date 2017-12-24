set -ex

cargo install --force cargo-travis
export PATH="$HOME/.cargo/bin:$PATH"

pip install 'travis-cargo<0.2' --user
export PATH="$HOME./local/bin:$PATH"
