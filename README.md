#

MBA 1.2Ghz 4-core i5 (rayon threads: 4)

RUST_LOG=info cargo bench:

```
baseline                time:   [992.97 ms 1.0109 s 1.0307 s] 
pmap 8096               time:   [969.04 ms 987.39 ms 1.0081 s]
pmap 16192              time:   [951.67 ms 976.07 ms 1.0036 s]
pmap 32384              time:   [957.40 ms 972.67 ms 990.39 ms]  

```