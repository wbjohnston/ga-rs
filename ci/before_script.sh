set -ex

# install travis-cargo and add to path
pip install 'travis-cargo<0.2' --user
export PATH="$HOME./local/bin:$PATH"
