#!/usr/bin/env bash
# This script runs an end-to-end test to ensure that tome
# works for the shell and operating system used.
#
# Although bash is mentioned, this is designed to work with
# multiple shells.
#
# requirements:
# - ran from the root of the tome repository
set -e # add -x to get verbose output
SHELL="$1"
export SHELL

cargo build --release
# init the tome in question
# $SHELL doesn't work in this context, albeit preferable,
# as it's not available during GitHub workf>low runs.
eval "$(./target/release/tome init e ./example "$SHELL")"

test_failures=0

# regular script example
GOT=$(e file_example)
WANT="hello"
if [ "${GOT}" != "${WANT}" ]; then
    echo "regular script failed; Got '${GOT}', want '${WANT}'"
    (( test_failures += 1 ))
fi

pushd .
# test source example
WANT="production"
unset DEV_ENVIRONMENT
e source_example
if [ "${DEV_ENVIRONMENT}" != "${WANT}" ]; then
    echo "e source_example failed to set DEV_ENVIRONMENT variable. Got '${DEV_ENVIRONMENT}', want '${WANT}'."
    (( test_failures += 1 ))
fi
popd

# regular nested script example
GOT=$(e dir_example bar)
WANT="bar"
if [ "${GOT}" != "${WANT}" ]; then
    echo "regular nested script failed; Got '${GOT}', want '${WANT}'"
    (( test_failures += 1 ))
fi

# test `TOME_SCRIPTS_ROOT`.
GOT=$(e read-from-root)
WANT="data"
if [ "${GOT}" != "${WANT}" ]; then
    echo "read-from-root failed; Got '${GOT}', want '${WANT}'"
    (( test_failures += 1 ))
fi

# ignore script with leading .'s example
if e help | grep -E '^ \.foo'; then
    echo ".script is ignored failed, found .foo; Got '$(e help)'"
    (( test_failures += 1 ))
fi

if (( test_failures > 0 )); then
    echo "Encountered ${test_failures} test failure(s)"
    exit 1
fi
echo Pass
