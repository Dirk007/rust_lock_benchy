#!/usr/bin/env bash

set -eu

source shell/include

READS=20000
WRITE_MOD=5000
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
