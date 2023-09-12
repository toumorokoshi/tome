#!/usr/bin/env bash
# This script runs an end-to-end test using `tome run`.
#
# requirements:
# - ran from the root of the tome repository
set -e # add -x to get verbose output
cargo build --release

GOT=$(./target/release/tome run ./example -- dir_example bar)
WANT="bar"
if [ "${GOT}" != "${WANT}" ]; then
    echo "tome run nested script failed; Got '${GOT}', want '${WANT}'"
fi

# test `TOME_SCRIPTS_ROOT`.
GOT=$(./target/release/tome run ./example -- read-from-root)
WANT="data"
if [ "${GOT}" != "${WANT}" ]; then
    echo "tome run read-from-root failed; Got '${GOT}', want '${WANT}'"
fi