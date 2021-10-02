#!/bin/bash

# Based on https://github.com/tiziano88/rust-raspberry-pi/blob/master/Dockerfile
# I just didn't feel like fussing with docker

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
DEST_DIR=$SCRIPT_DIR/rpi_tools
RPI_TOOLS_COMMIT_ID=5caa7046982f0539cf5380f94da04b31129ed521
ARCH_PATH="${DEST_DIR}/tools-${RPI_TOOLS_COMMIT_ID}/arm-bcm2708"

if [ ! -d $DEST_DIR ]; then

  echo "Installing RPI specific build tools to $DEST_DIR"
  mkdir $DEST_DIR

  curl -L https://github.com/raspberrypi/tools/archive/$RPI_TOOLS_COMMIT_ID.tar.gz > $DEST_DIR/rpi_tools.tar.gz

  cd $DEST_DIR
  tar --strip-components=1 -vxzf rpi_tools.tar.gz
fi


