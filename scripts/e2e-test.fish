#!/usr/bin/env fish
# This script runs and end-to-end test to ensure that tome
# works for the shell and operating system used.
#
# fish is wildly different from other shells (it's non-POSIX compliant),
# so a separate testing script is required.
#
# requirements:
# - ran from the root of the tome repository
# uncomment to get verbose output for debugging
# set -x
set SHELL "$argv[1]"
export SHELL

cargo build --release
# init the tome in question
# $SHELL doesn't work in this context, albeit preferable,
# as it's not available during GitHub workflow runs.
./target/release/tome init e ./example $SHELL | source

# regular script example
set GOT (e file_example)
set WANT "hello"
if [ "$GOT" != "$WANT" ];
    echo "regular script failed; Got '$GOT', want '$WANT'"
end

pushd .
# test source example
set WANT "production"
set DEV_ENVIRONMENT ""
e source_example_fish
if [ "$DEV_ENVIRONMENT" != "$WANT" ];
    echo "e source_example failed to set DEV_ENVIRONMENT variable. Got '$DEV_ENVIRONMENT', want '$WANT'."
end
popd

# regular nested script example
set GOT (e dir_example bar)
set WANT "bar"
if [ "$GOT" != "$WANT" ];
    echo "regular nested script failed; Got '$GOT', want '$WANT'"
end