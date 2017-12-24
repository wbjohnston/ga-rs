set -ex

# Upload docs
travis-cargo --only stable doc-upload

# Upload code coverage to coveralls
travis-cargo coveralls --no-sudo --verify
