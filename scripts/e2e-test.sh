#!/usr/bin/env bash
# This script runs and end-to-end test to ensure that tome
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
# as it's not available during GitHub workflow runs.
eval "$(./target/release/tome init e ./example $SHELL)"

# regular script example
GOT=$(e file_example)
WANT="hello"
if [ "${GOT}" != "${WANT}" ]; then
    echo "regular script failed; Got '${GOT}', want '${WANT}'"
fi

pushd .
# test source example
WANT="production"
unset DEV_ENVIRONMENT
e source_example
if [ "${DEV_ENVIRONMENT}" != "${WANT}" ]; then
    echo "e source_example failed to set DEV_ENVIRONMENT variable. Got '${DEV_ENVIRONMENT}', want '${WANT}'."
fi
popd

# regular nested script example
GOT=$(e dir_example bar)
WANT="bar"
if [ "${GOT}" != "${WANT}" ]; then
    echo "regular nested script failed; Got '${GOT}', want '${WANT}'"
fi