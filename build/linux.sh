#!/bin/bash

ME="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
PROJECT_DIR="$( realpath "$ME/../" )"

cd $PROJECT_DIR

cargo install cargo-config
cargo install cargo-deb

PROJECT_NAME="$( cargo config package.name | cut -d '"' -f 2 )"
PROJECT_VERSION="$( cargo config package.version | cut -d '"' -f 2 )"

if [ -z "$BUILD_NUMBER" ]; then DEBIAN_VERSION=$PROJECT_VERSION; else DEBIAN_VERSION="$PROJECT_VERSION$BUILD_NUMBER"; fi

cargo clean



cargo build --bin psistats --release
cargo deb --deb-version $DEBIAN_VERSION

