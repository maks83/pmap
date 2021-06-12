#
test map function calculates md5 from 4kb random string
N = 100k strings

Spec: MBA 1.2Ghz 4-core i5 (rayon threads: 4)

RUST_LOG=info cargo bench:

```
baseline                time:   [926.94 ms 938.35 ms 950.81 ms]
pmap 1024               time:   [391.82 ms 399.31 ms 407.63 ms]
pmap 2048               time:   [388.70 ms 392.06 ms 395.75 ms]
pmap 4096               time:   [390.24 ms 395.48 ms 401.48 ms]  
pmap 8192               time:   [400.83 ms 408.76 ms 418.40 ms]     
pmap 16384              time:   [426.17 ms 434.10 ms 443.49 ms]        
pmap 32768              time:   [425.76 ms 433.50 ms 441.80 ms]       

```