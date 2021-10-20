#!/usr/bin/env bash

set -eu

source shell/include

echo "Building..."
for WHAT in ${TYPES[*]}
do
    echo "- ${WHAT}"
    cargo build --release --features="$WHAT" --no-default-features 1>/dev/null 2>&1
    BENCH_VERSION="$(target/release/test_mux --version)"
    mv target/release/test_mux test_mux_$WHAT
done

echo "Collecting informations..."
echo "* BENCH VERSION: ${BENCH_VERSION}"
echo "* CPU:"
CPU="$(lscpu 2>/dev/null || true)"
if [ -z $CPU ]; then
    CPU="$(sysctl -n machdep.cpu.brand_string  2>/dev/null || true)"
fi
echo $CPU

echo "* System:"
SYSTEM="$(uname -mrs || true)"

TARGET="$(echo "$SYSTEM $CPU" | tr " " "_" | tr "/" "_" | tr "\\" "_")"
echo "$TARGET"

TARGET="benchmarks/${TARGET}_bench.txt"
echo "* Benching to ${TARGET}"

echo "BENCH VERSION: ${BENCH_VERSION}" > ${TARGET}
echo "CPU: ${CPU}" >> ${TARGET}
echo "SYSTEM: ${SYSTEM}" >> ${TARGET}
echo "" >> ${TARGET}
./bench.sh | tee -a ${TARGET}

echo "Cleanup"
for WHAT in ${TYPES[*]}
do
    rm test_mux_$WHAT
done
