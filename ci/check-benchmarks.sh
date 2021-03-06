#!/bin/bash

set -x;

bash -c "while true; do sleep 30; echo \$(date) - running ...; done" &
PING_LOOP_PID=$!
trap - ERR
RUST_BACKTRACE=1 RUST_LOG=collector_raw_cargo=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    bench_test $BENCH_TEST_OPTS
code=$?
kill $PING_LOOP_PID
exit $code
