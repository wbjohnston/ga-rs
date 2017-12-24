set -ex

# Upload docs
travis-cargo --only stable doc-upload

cargo coveralls
