#!/bin/bash
set -x
set -e
TARGET=$1
PROJECT_NAME="$( cargo config package.name | cut -d '"' -f 2 )"
PROJECT_VERSION="$( cargo config package.version | cut -d '"' -f 2 )"

declare -A TARGET_MAP
TARGET_MAP["armv7-unknown-linux-gnueabihf"] = "armhf"
TARGET_MAP["x86_64-unknown-linux-gnu"] = "amd64"
PROJECT_DEB_ARCH=${TARGET_MAP["${TARGET}"]}


ME="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
PROJECT_DIR="$( realpath "$ME/../" )"
cd $PROJECT_DIR
mkdir -p target/artifacts

if [ -z "$BUILD_NUMBER" ]; then DEBIAN_VERSION=$PROJECT_VERSION; else DEBIAN_VERSION="$PROJECT_VERSION$BUILD_NUMBER"; fi

DEBIAN_TARGET_DIR="$PROJECT_DIR/target/$TARGET/debian"
DEBIAN_FILE="$DEBIAN_TARGET_DIR/${PROJECT_NAME}_${DEBIAN_VERSION}_${PROJECT_DEB_ARCH}.deb"

cargo deb --deb-version $DEBIAN_VERSION --target $PLATFORM

cd $DEBIAN_TARGET_DIR

ar x $DEBIAN_FILE

cat debian-binary control.tar.gz data.tar.xz > "/tmp/${PROJECT_NAME}-${DEBIAN_VERSION}-combined-contents"
gpg -abs -o _gpuorigin "/tmp/${PROJECT_NAME}-${DEBIAN_VERSION}-combined-contents"
ar rc $DEBIAN_FILE _gpuorigin debian-binary control.tar.gz data.tar.xz
rm _gpuorigin debian-binary control.tar.gz data.tar.xz "/tmp/${PROJECT_NAME}-${DEBIAN_VERSION}-combined-contents"
mv $DEBIAN_FILE "${PROJECT_DIR}/target/artifacts"

aptly repo add psikon-devel $DEBIAN_FILE
~/debian_repo/update.sh
