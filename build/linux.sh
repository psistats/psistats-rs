#!/bin/bash

ME="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
PROJECT_DIR="$( realpath "$ME/../" )"

cd $PROJECT_DIR

cargo clean
cargo build --bin psistats --release
cargo deb

