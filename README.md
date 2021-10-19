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

## Results for Linux
CPU 0..7:
```
processor   : 0
vendor_id   : GenuineIntel
cpu family  : 6
model       : 142
model name  : Intel(R) Core(TM) i7-8550U CPU @ 1.80GHz
stepping    : 10
microcode   : 0xea
cpu MHz     : 800.051
cache size  : 8192 KB
physical id : 0
siblings    : 8
core id     : 0
cpu cores   : 4
apicid      : 0
initial apicid  : 0
fpu     : yes
fpu_exception   : yes
cpuid level : 22
wp      : yes
flags       : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb invpcid_single pti ssbd ibrs ibpb stibp tpr_shadow vnmi flexpriority ept vpid ept_ad fsgsbase tsc_adjust sgx bmi1 avx2 smep bmi2 erms invpcid mpx rdseed adx smap clflushopt intel_pt xsaveopt xsavec xgetbv1 xsaves dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp md_clear flush_l1d
vmx flags   : vnmi preemption_timer invvpid ept_x_only ept_ad ept_1gb flexpriority tsc_offset vtpr mtf vapic ept vpid unrestricted_guest ple pml ept_mode_based_exec
bugs        : cpu_meltdown spectre_v1 spectre_v2 spec_store_bypass l1tf mds swapgs itlb_multihit srbds
bogomips    : 4001.60
clflush size    : 64
cache_alignment : 64
address sizes   : 39 bits physical, 48 bits virtual
power management:
```
```
Linux 5.14.9-arch2-1 #1 SMP PREEMPT Fri, 01 Oct 2021 19:03:20 +0000 x86_64 GNU/Linux
```

```mutex> (10 samples) took 71.247ms (9 readers, 1 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 71.361ms (8 readers, 2 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 70.974ms (6 readers, 4 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 355.918ms (49 readers, 1 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 355.937ms (48 readers, 2 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 357.630ms (46 readers, 4 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 718.536ms (99 readers, 1 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 718.968ms (98 readers, 2 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 725.173ms (96 readers, 4 writers, 40000 reads, 4 writes)
-------------------------------------
rwlock> (10 samples) took 57.570ms (9 readers, 1 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 67.101ms (8 readers, 2 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 70.918ms (6 readers, 4 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 438.778ms (49 readers, 1 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 457.505ms (48 readers, 2 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 468.849ms (46 readers, 4 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 916.638ms (99 readers, 1 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 921.789ms (98 readers, 2 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 879.040ms (96 readers, 4 writers, 40000 reads, 4 writes)
-------------------------------------
pwlock> (10 samples) took 11.709ms (9 readers, 1 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 11.476ms (8 readers, 2 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 10.192ms (6 readers, 4 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 44.738ms (49 readers, 1 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 41.729ms (48 readers, 2 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 40.844ms (46 readers, 4 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 77.676ms (99 readers, 1 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 78.554ms (98 readers, 2 writers, 40000 reads, 4 writes)
pwlock> (10 samples) took 82.872ms (96 readers, 4 writers, 40000 reads, 4 writes)```