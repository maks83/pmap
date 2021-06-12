#
test map function calculates md5 from 4kb random string
N = 100k strings

Spec: MBA 1.2Ghz 4-core i5 (rayon threads: 4)

RUST_LOG=info cargo bench:

```
baseline                time:   [1.1168 s 1.1246 s 1.1336 s]   
pmap 256                time:   [571.87 ms 580.24 ms 591.50 ms]   
pmap 512                time:   [575.65 ms 585.45 ms 596.24 ms]   
pmap 1024               time:   [541.13 ms 549.90 ms 560.55 ms]        
pmap 2048               time:   [561.87 ms 576.67 ms 594.00 ms]     
pmap 4096               time:   [556.99 ms 568.08 ms 579.94 ms]  
pmap 8192               time:   [529.32 ms 536.82 ms 545.05 ms]     
pmap 16384              time:   [560.47 ms 567.73 ms 575.47 ms]      
pmap 32768              time:   [478.92 ms 489.91 ms 501.99 ms]          

```