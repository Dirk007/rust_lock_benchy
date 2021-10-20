# Rust lock-bencher

Just a little test to see how different lock-implementations in rust work out in different situations.

Use like `./run_tests.sh` to run all tests at all.

## Locks used in this benchmark

- tokio::sync::Mutex
- tokio::sync::RwLock
- [Parking Lot RwLock](https://github.com/Amanieu/parking_lot)

## Contribution
Feel free to PR changes for whatever you want to test additionally.
Also bench-outputs for different systems are very welcome.

## Results for a MacBook Air M1 (aarch64):

### CPU
```
CPU-Info: sysctl -a | grep machdep.cpu
machdep.cpu.brand_string: Apple M1
machdep.cpu.core_count: 8
machdep.cpu.cores_per_package: 8
machdep.cpu.logical_per_package: 8
machdep.cpu.thread_count: 8
```

### OS

### Bench

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

## Results for x64 Linux

### CPU 
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

### OS
```
Linux 5.14.9-arch2-1 #1 SMP PREEMPT Fri, 01 Oct 2021 19:03:20 +0000 x86_64 GNU/Linux
```

### Bench
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
pwlock> (10 samples) took 82.872ms (96 readers, 4 writers, 40000 reads, 4 writes)
```

## Results for x64 MacOS

Note: Used v 0.2.0 - pwlock == plrwlock

### CPU
```machdep.cpu.brand_string: Intel(R) Core(TM) i7-8750H CPU @ 2.20GHz
machdep.cpu.core_count: 6
machdep.cpu.cores_per_package: 8
machdep.cpu.extfamily: 0
machdep.cpu.extfeature_bits: 1241984796928
machdep.cpu.extfeatures: SYSCALL XD 1GBPAGE EM64T LAHF LZCNT PREFETCHW RDTSCP TSCI
machdep.cpu.extmodel: 9
machdep.cpu.family: 6
machdep.cpu.feature_bits: 9221959987971750911
machdep.cpu.features: FPU VME DE PSE TSC MSR PAE MCE CX8 APIC SEP MTRR PGE MCA CMOV PAT PSE36 CLFSH DS ACPI MMX FXSR SSE SSE2 SS HTT TM PBE SSE3 PCLMULQDQ DTES64 MON DSCPL VMX EST TM2 SSSE3 FMA CX16 TPR PDCM SSE4.1 SSE4.2 x2APIC MOVBE POPCNT AES PCID XSAVE OSXSAVE SEGLIM64 TSCTMR AVX1.0 RDRAND F16C
machdep.cpu.leaf7_feature_bits: 43804591 1073741824
machdep.cpu.leaf7_feature_bits_edx: 2617257472
machdep.cpu.leaf7_features: RDWRFSGS TSC_THREAD_OFFSET SGX BMI1 AVX2 SMEP BMI2 ERMS INVPCID FPU_CSDS MPX RDSEED ADX SMAP CLFSOPT IPT SGXLC MDCLEAR TSXFA IBRS STIBP L1DF SSBD
machdep.cpu.logical_per_package: 16
```

### OS
```
Darwin Dirks-CS-MacBook-Pro-606.local 20.6.0 Darwin Kernel Version 20.6.0: Mon Aug 30 06:12:21 PDT 2021; root:xnu-7195.141.6~3/RELEASE_X86_64 x86_64
```

### Bench

```
mutex> (10 samples) took 74.312ms (9 readers, 1 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 76.176ms (8 readers, 2 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 73.912ms (6 readers, 4 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 371.744ms (49 readers, 1 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 367.077ms (48 readers, 2 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 368.709ms (46 readers, 4 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 731.420ms (99 readers, 1 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 736.265ms (98 readers, 2 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 732.093ms (96 readers, 4 writers, 40000 reads, 4 writes)
-------------------------------------
rwlock> (10 samples) took 78.470ms (9 readers, 1 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 82.941ms (8 readers, 2 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 82.762ms (6 readers, 4 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 412.065ms (49 readers, 1 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 450.002ms (48 readers, 2 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 446.116ms (46 readers, 4 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 946.772ms (99 readers, 1 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 951.343ms (98 readers, 2 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 984.430ms (96 readers, 4 writers, 40000 reads, 4 writes)
-------------------------------------
plmutex> (10 samples) took 12.797ms (9 readers, 1 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 12.807ms (8 readers, 2 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 13.443ms (6 readers, 4 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 69.727ms (49 readers, 1 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 70.989ms (48 readers, 2 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 69.954ms (46 readers, 4 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 144.242ms (99 readers, 1 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 143.459ms (98 readers, 2 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 140.084ms (96 readers, 4 writers, 40000 reads, 4 writes)
-------------------------------------
plrwlock> (10 samples) took 7.089ms (9 readers, 1 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 6.717ms (8 readers, 2 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 7.223ms (6 readers, 4 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 31.223ms (49 readers, 1 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 32.993ms (48 readers, 2 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 30.725ms (46 readers, 4 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 60.291ms (99 readers, 1 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 59.701ms (98 readers, 2 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 59.422ms (96 readers, 4 writers, 40000 reads, 4 writes)
-------------------------------------
```

## Results for AARCH64 Linux

### CPU
CPU 0..3 (overclocked - vcgencmd measure_clock arm -> frequency(48)=60016992)
```
processor	: 0
BogoMIPS	: 108.00
Features	: fp asimd evtstrm crc32 cpuid
CPU implementer	: 0x41
CPU architecture: 8
CPU variant	: 0x0
CPU part	: 0xd08
CPU revision	: 3
```

### OS
```
Linux pi64 5.10.60-v8+ #1449 SMP PREEMPT Wed Aug 25 15:01:33 BST 2021 aarch64 GNU/Linux
```

### Bench

```
mutex> (10 samples) took 213.895ms (9 readers, 1 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 215.289ms (8 readers, 2 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 218.373ms (6 readers, 4 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 1073.515ms (49 readers, 1 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 1069.438ms (48 readers, 2 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 1077.378ms (46 readers, 4 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 2184.096ms (99 readers, 1 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 2159.106ms (98 readers, 2 writers, 40000 reads, 4 writes)
mutex> (10 samples) took 2159.851ms (96 readers, 4 writers, 40000 reads, 4 writes)
-------------------------------------
rwlock> (10 samples) took 77.594ms (9 readers, 1 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 72.166ms (8 readers, 2 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 72.627ms (6 readers, 4 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 381.925ms (49 readers, 1 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 377.900ms (48 readers, 2 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 374.039ms (46 readers, 4 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 759.564ms (99 readers, 1 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 754.292ms (98 readers, 2 writers, 40000 reads, 4 writes)
rwlock> (10 samples) took 748.338ms (96 readers, 4 writers, 40000 reads, 4 writes)
-------------------------------------
plmutex> (10 samples) took 24.723ms (9 readers, 1 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 24.875ms (8 readers, 2 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 24.333ms (6 readers, 4 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 123.766ms (49 readers, 1 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 123.180ms (48 readers, 2 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 127.554ms (46 readers, 4 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 251.107ms (99 readers, 1 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 258.295ms (98 readers, 2 writers, 40000 reads, 4 writes)
plmutex> (10 samples) took 258.163ms (96 readers, 4 writers, 40000 reads, 4 writes)
-------------------------------------
plrwlock> (10 samples) took 12.736ms (9 readers, 1 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 13.207ms (8 readers, 2 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 12.545ms (6 readers, 4 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 60.171ms (49 readers, 1 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 61.631ms (48 readers, 2 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 61.773ms (46 readers, 4 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 120.902ms (99 readers, 1 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 123.718ms (98 readers, 2 writers, 40000 reads, 4 writes)
plrwlock> (10 samples) took 122.591ms (96 readers, 4 writers, 40000 reads, 4 writes)
-------------------------------------
```
