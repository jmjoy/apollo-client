#!/usr/bin/env bash

set -xe

TMP_CARGO_CLIENT_DIR=/tmp/apollo-client

CARGO_INCREMENTAL=0 RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads" cargo +nightly test --features full
mkdir -p ${TMP_CARGO_CLIENT_DIR}
zip -0 ${TMP_CARGO_CLIENT_DIR}/ccov.zip `find . \( -name "apollo*.gc*" \) -print`
grcov ${TMP_CARGO_CLIENT_DIR}/ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" -o ${TMP_CARGO_CLIENT_DIR}/lcov.info
genhtml -o ${TMP_CARGO_CLIENT_DIR}/report/ --show-details --highlight --ignore-errors source --legend ${TMP_CARGO_CLIENT_DIR}/lcov.info
xdg-open ${TMP_CARGO_CLIENT_DIR}/report/index.html
