#!/bin/bash
set -e
TARGET=$1
PROJECT_NAME="psistats"
PROJECT_VERSION="0.3.0-beta"

if [ -z "$TARGET" ]
then
    TARGET="x86_64-unknown-linux-gnu"
fi

declare -A target_map
target_map["armv7-unknown-linux-gnueabihf"]="armhf"
target_map["x86_64-unknown-linux-gnu"]="amd64"
target_map["aarch64-unknown-linux-gnu"]="arm64"

declare -A target_ar_map
target_ar_map["armv7-unknown-linux-gnueabihf"]="/usr/bin/arm-linux-gnueabihf-gcc-ar"
target_ar_map["x86_64-unknown-linux-gnu"]="/usr/bin/x86_64-linux-gnu-gcc-ar"
target_ar_map["aarch64-unknown-linux-gnu"]="/usr/bin/aarch64-linux-gnu-gcc-ar"

declare -A target_cc_map
target_cc_map["armv7-unknown-linux-gnueabihf"]="/usr/bin/arm-linux-gnueabihf-gcc"
target_cc_map["x86_64-unknown-linux-gnu"]="/usr/bin/x86_64-linux-gnu-gcc"
target_cc_map["aarch64-unknown-linux-gnu"]="/usr/bin/aarch64-linux-gnu-gcc"

if [ -z ${target_map[$TARGET]+"check"} ]; then
  echo "Invalid target architecture. Must be one of: "
  for i in "${!target_map[@]}"
  do
    echo "    $i"
  done
  exit 1
fi
PROJECT_ARCH=${target_map[$TARGET]}
export TARGET_AR=${target_ar_map[$TARGET]}
export TARGET_CC=${target_cc_map[$TARGET]}

echo "Target AR: ${TARGET_AR}"
echo "Target CC: ${TARGET_CC}"

rustup target add ${TARGET}


ME="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
PROJECT_DIR="$( realpath "$ME/../../" )"

export RUSTFLAGS=""

if [ $PROJECT_ARCH == "armhf" ]; then
  RUSTFLAGS="$RUSTFLAGS -L $ME/3rdparty/libsensors/lib"
fi

if [ $PROJECT_ARCH == "arm64" ]; then
  RUSTFLAGS="$RUSTFLAGS -L $ME/3rdparty/libsensors-arm64/lib"
fi


cd $PROJECT_DIR
cargo install cargo-deb

cd $PROJECT_DIR/psistats

#PROJECT_NAME="$( cargo config package.name | cut -d '"' -f 2 )"
#PROJECT_VERSION="$( cargo config package.version | cut -d '"' -f 2 )"
if [ -z "$BUILD_NUMBER" ]; then DEBIAN_VERSION=$PROJECT_VERSION; else DEBIAN_VERSION="${PROJECT_VERSION}-${BUILD_NUMBER}"; fi

cd $PROJECT_DIR

echo Project location: $PROJECT_DIR
echo Project name: $PROJECT_NAME
echo Project version: $PROJECT_VERSION
echo Debian version: $DEBIAN_VERSION

RELEASE_DIR=${PROJECT_DIR}/target/$TARGET/release
ARTIFACT_DIR=${RELEASE_DIR}/artifacts
UNZIPPED_DIR=${RELEASE_DIR}/unzipped/${PROJECT_NAME}-${DEBIAN_VERSION}-${PROJECT_ARCH}

rm -rf $ARTIFACT_DIR
rm -rf ${RELEASE_DIR}/unzipped

#                   /3rdparty/libsensors4-dev-armhf/usr/include

# RUSTFLAGS="-l sensors -L ${ME}/3rdparty/libsensors4-dev-armhf/usr/include/sensors" cargo build --target $TARGET --release
echo "RUST FLAGS: $RUSTFLAGS"
cargo build --target $TARGET --release

mkdir -p $UNZIPPED_DIR
mkdir -p $ARTIFACT_DIR
mkdir -p ${UNZIPPED_DIR}/plugins
cp $RELEASE_DIR/psistats $UNZIPPED_DIR
cp $RELEASE_DIR/libplugin_*.so ${UNZIPPED_DIR}/plugins
cp $PROJECT_DIR/LICENSE $UNZIPPED_DIR
cp $PROJECT_DIR/psistats.toml $UNZIPPED_DIR

cd $RELEASE_DIR/unzipped

tar -cvzf ${ARTIFACT_DIR}/${PROJECT_NAME}_${DEBIAN_VERSION}_${PROJECT_ARCH}.tar.gz *

cd $PROJECT_DIR

DEBIAN_DIR=${RELEASE_DIR}/debian
mkdir -p $DEBIAN_DIR

cargo deb --deb-version=${DEBIAN_VERSION} --target ${TARGET} -p psistats

DEBIAN_TARGET_DIR=${PROJECT_DIR}/target/${TARGET}/debian
DEBIAN_FILE=${DEBIAN_TARGET_DIR}/${PROJECT_NAME}_${DEBIAN_VERSION}_${target_map[$TARGET]}.deb

cp $DEBIAN_FILE $ARTIFACT_DIR
mkdir -p ${PROJECT_DIR}/target/release/artifacts
cp -r ${ARTIFACT_DIR}/* ${PROJECT_DIR}/target/release/artifacts
