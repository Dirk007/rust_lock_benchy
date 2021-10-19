# Rust lock-bencher

Just a little test to see how different lock-implementations in rust work out in different situations.

Use like `./run_tests.sh` to run all tests at all.

## Results for a MacBook Air M1 (aarch64):
```
CPU-Info: sysctl -a | grep machdep.cpu
machdep.cpu.brand_string: Apple M1
machdep.cpu.core_count: 8
machdep.cpu.cores_per_package: 8
machdep.cpu.logical_per_package: 8
machdep.cpu.thread_count: 8
```

```
mutex> (10 samples) took 33.588ms (9 readers, 1 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 30.864ms (8 readers, 2 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 30.886ms (6 readers, 4 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 154.350ms (49 readers, 1 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 154.731ms (48 readers, 2 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 154.261ms (46 readers, 4 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 308.474ms (99 readers, 1 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 308.786ms (98 readers, 2 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 309.167ms (96 readers, 4 writers, 40000 reads, 4 writes)
-------------------------------------
rwlock> (10 samples) took 27.879ms (9 readers, 1 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 26.619ms (8 readers, 2 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 25.798ms (6 readers, 4 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 115.351ms (49 readers, 1 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 91.874ms (48 readers, 2 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 138.305ms (46 readers, 4 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 211.632ms (99 readers, 1 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 266.713ms (98 readers, 2 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 275.317ms (96 readers, 4 writers, 40000 reads, 4 writes)
-------------------------------------
pwlock> (10 samples) took 32.105ms (9 readers, 1 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 31.713ms (8 readers, 2 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 31.583ms (6 readers, 4 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 187.666ms (49 readers, 1 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 186.966ms (48 readers, 2 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 185.583ms (46 readers, 4 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 381.928ms (99 readers, 1 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 370.600ms (98 readers, 2 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 376.998ms (96 readers, 4 writers, 40000 reads, 4 writes)
```


