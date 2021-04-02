The benchmark compares encoding and decoding times of gzip, lz4 and zstd
with different compression levels on a SCALE encoded kusama.compact.wasm file.

TL;DR: looks like `zstd` offers better results for our use-case than gzip.

Running `cargo bench` on my laptop yields:

```
encode/gzip(4)          time:   [38.492 ms 38.731 ms 39.043 ms]                           
                        thrpt:  [54.891 MiB/s 55.332 MiB/s 55.676 MiB/s]
encode/gzip(6)          time:   [96.017 ms 96.791 ms 97.661 ms]                           
                        thrpt:  [21.944 MiB/s 22.141 MiB/s 22.320 MiB/s]
encode/lz4(1)           time:   [5.9714 ms 6.0003 ms 6.0439 ms]                          
                        thrpt:  [354.59 MiB/s 357.16 MiB/s 358.89 MiB/s]
encode/lz4(4)           time:   [32.601 ms 32.737 ms 32.953 ms]                          
                        thrpt:  [65.035 MiB/s 65.464 MiB/s 65.737 MiB/s]
encode/zstd(3)          time:   [11.039 ms 11.094 ms 11.183 ms]                           
                        thrpt:  [191.63 MiB/s 193.18 MiB/s 194.14 MiB/s]
encode/zstd(10)         time:   [71.690 ms 72.203 ms 72.785 ms]                            
                        thrpt:  [29.444 MiB/s 29.681 MiB/s 29.894 MiB/s]

Compression ratios:
  gzip(4): 3.1345544
  gzip(6): 3.3006585
  lz4(1): 2.1750674
  lz4(4): 2.7585163
  zstd(3): 3.4258308
  zstd(10): 3.7935927

decode/gzip(4)          time:   [7.2596 ms 7.3016 ms 7.3524 ms]                           
decode/gzip(6)          time:   [7.1225 ms 7.1624 ms 7.2133 ms]                           
decode/lz4(1)           time:   [1.4237 ms 1.4536 ms 1.4896 ms]                           
decode/lz4(4)           time:   [1.5688 ms 1.5763 ms 1.5848 ms]                           
decode/zstd(3)          time:   [2.5232 ms 2.5384 ms 2.5541 ms]                            
decode/zstd(10)         time:   [2.2866 ms 2.2993 ms 2.3133 ms]                             
decode/ruzstd(3)        time:   [14.543 ms 14.605 ms 14.678 ms]                             

```
