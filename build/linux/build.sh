#!/bin/bash
set -e
TARGET=$1

declare -A target_map
target_map["armv7-unknown-linux-gnueabihf"]="armhf"
target_map["x86_64-unknown-linux-gnu"]="amd64"

if [ -z ${target_map[$TARGET]+"check"} ]; then
  echo "Invalid target architecture. Must be one of: "
  for i in "${!target_map[@]}"
  do
    echo "    $i"
  done
  exit 1
fi
PROJECT_DEB_ARCH=${target_map[$TARGET]}

ME="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
PROJECT_DIR="$( realpath "$ME/../../" )"
cd $PROJECT_DIR

cargo install cargo-config cargo-deb

cd $PROJECT_DIR/psistats

PROJECT_NAME="$( cargo config package.name | cut -d '"' -f 2 )"
PROJECT_VERSION="$( cargo config package.version | cut -d '"' -f 2 )"

cd $PROJECT_DIR

echo Project location: $PROJECT_DIR
echo Project name: $PROJECT_NAME
echo Project version: $PROJECT_VERSION

RELEASE_DIR=$PROJECT_DIR/target/release
ARTIFACT_DIR=$RELEASE_DIR/artifacts
UNZIPPED_DIR=$RELEASE_DIR/unzipped/$PROJECT_NAME-$PROJECT_VERSION

rm -rf $ARTIFACT_DIR
rm -rf $RELEASE_DIR/unzipped

cargo build --release

mkdir -p $UNZIPPED_DIR
mkdir -p $ARTIFACT_DIR
mkdir -p $UNZIPPED_DIR/plugins
cp $RELEASE_DIR/psistats $UNZIPPED_DIR
cp $RELEASE_DIR/libplugin_*.so $UNZIPPED_DIR/plugins
cp $PROJECT_DIR/LICENSE $UNZIPPED_DIR
cp $PROJECT_DIR/psistats.toml $UNZIPPED_DIR

cd $RELEASE_DIR/unzipped

tar -cvzf $ARTIFACT_DIR/$PROJECT_NAME-$PROJECT_VERSION.tar.gz *

cd $PROJECT_DIR

DEBIAN_DIR=$RELEASE_DIR/debian
mkdir -p $DEBIAN_DIR

cargo deb --deb-version $PROJECT_VERSION --target $TARGET -p psistats

DEBIAN_TARGET_DIR=$PROJECT_DIR/target/$TARGET/debian
DEBIAN_FILE=$DEBIAN_TARGET_DIR/${PROJECT_NAME}_${PROJECT_VERSION}_${target_map[$TARGET]}.deb

cp $DEBIAN_FILE $ARTIFACT_DIR

