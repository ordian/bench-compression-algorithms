The benchmark compares encoding and decoding times of gzip, lz4 and zstd
with different compression levels on a SCALE encoded kusama.compact.wasm file.

TL;DR: looks like `zstd` offers better results for our use-case than gzip.

Running `cargo bench` on my laptop yields:

```
encode/gzip(4)          time:   [38.492 ms 38.731 ms 39.043 ms]                           
                        thrpt:  [54.891 MiB/s 55.332 MiB/s 55.676 MiB/s]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
Benchmarking encode/gzip(6): Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.5s, or reduce sample count to 50.
encode/gzip(6)          time:   [96.017 ms 96.791 ms 97.661 ms]                           
                        thrpt:  [21.944 MiB/s 22.141 MiB/s 22.320 MiB/s]
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe
encode/lz4(1)           time:   [5.9714 ms 6.0003 ms 6.0439 ms]                          
                        thrpt:  [354.59 MiB/s 357.16 MiB/s 358.89 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
encode/lz4(4)           time:   [32.601 ms 32.737 ms 32.953 ms]                          
                        thrpt:  [65.035 MiB/s 65.464 MiB/s 65.737 MiB/s]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
encode/zstd(3)          time:   [11.039 ms 11.094 ms 11.183 ms]                           
                        thrpt:  [191.63 MiB/s 193.18 MiB/s 194.14 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
Benchmarking encode/zstd(10): Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 7.3s, or reduce sample count to 60.
encode/zstd(10)         time:   [71.690 ms 72.203 ms 72.785 ms]                            
                        thrpt:  [29.444 MiB/s 29.681 MiB/s 29.894 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
Compression ratios:
  gzip(4): 3.1345544
  gzip(6): 3.3006585
  lz4(1): 2.1750674
  lz4(4): 2.7585163
  zstd(3): 3.4258308
  zstd(10): 3.7935927
```
