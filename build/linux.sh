#!/bin/bash

ME="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
PROJECT_DIR="$( realpath "$ME/../" )"

cd $PROJECT_DIR

cargo install cargo-config
cargo install cargo-deb

PROJECT_NAME="$( cargo config package.name | cut -d '"' -f 2 )"
PROJECT_VERSION="$( cargo config package.version | cut -d '"' -f 2 )"

if [ -z "$BUILD_NUMBER" ]; then DEBIAN_VERSION=$PROJECT_VERSION; else DEBIAN_VERSION="$PROJECT_VERSION$BUILD_NUMBER"; fi

DEBIAN_TARGET_DIR="$PROJECT_DIR/target/debian"
DEBIAN_FILE="$DEBIAN_TARGET_DIR/${PROJECT_NAME}_${DEBIAN_VERSION}_amd64.deb"

cargo clean
cargo build --bin psistats --release
cargo deb --deb-version $DEBIAN_VERSION --verbose

cd $DEBIAN_TARGET_DIR

ar x $DEBIAN_FILE

cat debian-binary control.tar.gz data.tar.xz > "/tmp/${PROJECT_NAME}-combined-contents"
gpg -abs -o _gpuorigin "/tmp/${PROJECT_NAME}-combined-contents"
ar rc $DEBIAN_FILE _gpuorigin debian-binary control.tar.gz data.tar.xz
rm _gpuorigin debian-binary control.tar.gz data.tar.xz
aptly repo add psikon-devel $DEBIAN_FILE
~/debian_repo/update.sh

