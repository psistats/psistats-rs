#!/bin/bash
set -e
set -x
COMPONENT=$1

ME="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
PROJECT_DIR="$( realpath "$ME/../../" )"
ARTIFACTS_DIR=$PROJECT_DIR/target/release/artifacts

DEBREPO_PATH=$HOME/Scripts/debrepo

cd $DEBREPO_PATH

echo "Artifacts directory: $ARTIFACTS_DIR"

#cd $ARTIFACTS_DIR

for f in *.deb; do
    DEB_FILE=$ARTIFACTS_DIR/$f
    echo "Debian file: $DEB_FILE"
    ./add.sh $COMPONENT $DEB_FILE
done

./publish.sh


