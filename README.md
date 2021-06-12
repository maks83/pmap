#
test map function calculates md5 from 4kb random string
N = 100k strings

Spec: MBA 1.2Ghz 4-core i5 (rayon threads: 4)

RUST_LOG=info cargo bench:

```
baseline                time:   [930.39 ms 938.00 ms 947.37 ms] 
pmap 256                time:   [354.57 ms 355.93 ms 357.36 ms]
pmap 512                time:   [379.67 ms 388.59 ms 399.59 ms]    
pmap 1024               time:   [362.59 ms 365.80 ms 369.55 ms]  
pmap 2048               time:   [377.31 ms 383.55 ms 390.78 ms]   
pmap 4096               time:   [401.23 ms 407.32 ms 414.41 ms]  
pmap 8192               time:   [426.99 ms 436.69 ms 448.63 ms]     
pmap 16384              time:   [463.65 ms 469.38 ms 475.97 ms]     
pmap 32768              time:   [478.92 ms 489.91 ms 501.99 ms]          

```