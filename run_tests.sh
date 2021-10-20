#!/usr/bin/env bash

set -eu

TYPES=("mutex" "rwlock" "plmutex" "plrwlock")

echo "Building..."
for WHAT in ${TYPES[*]}
do
    cargo build --release --features="$WHAT" --no-default-features
    mv target/release/test_mux test_mux_$WHAT
done

echo "Benching..."
READS=40000
WRITE_MOD=10000
for WHAT in ${TYPES[*]}
do
    for TASKS in 10 50 100
    do
        for WRITERS in 1 2 4
        do
            echo "$WHAT> $(./test_mux_${WHAT} --reads $READS --tasks $TASKS --writers $WRITERS --writemod $WRITE_MOD)"
        done
    done
    echo "-------------------------------------"
done

echo "Cleanup"
for WHAT in ${TYPES[*]}
do
    rm test_mux_$WHAT
done
