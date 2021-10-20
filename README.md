# Rust lock-bencher

Just a little test to see how different lock-implementations in rust work out in different situations.

Use like `./run_tests.sh` to run all tests at all.
Benchmark will be PR-ready saved to  the `benchmarks`-directory.

## Locks used in this benchmark

- std::sync::Mutex (mutex) + std::sync::RwLock (rwlock)
- tokio::sync::Mutex (tmutex) + tokio::sync::RwLock (trwlock)
- [Parking Lot](https://github.com/Amanieu/parking_lot) Mutex (plmutex) + RwLock (plrwlock)

## Contribution
Feel free to PR changes for whatever you want to test additionally.
Also PRs for bench-outputs on different systems are very welcome.

